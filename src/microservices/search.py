import requests
import torch
import json
from flask import Flask, request, jsonify
from flask_cors import CORS
from transformers import AutoTokenizer, AutoModelForCausalLM
from dotenv import load_dotenv
import os

# Microservice for search (gathering sources outside DDG) & summary
# (TinyGPT or other lightweight language model for reasonable local inference time).

# NOTE: Store this within a defined environment variable so users can supply their own.
# AIzaSyAj31A-XzkAWneP24wu4hPGYAhKLnDaUsk
load_dotenv()

API_KEY = os.getenv("API_KEY")
PROGRAMMABLE_ENGINE_KEY = os.getenv("PROGRAMMABLE_ENGINE_KEY")
MODEL = os.getenv("MODEL")

SEARCH_URL = "https://www.googleapis.com/customsearch/v1"

app = Flask(__name__)
CORS(app, resources={r"/*": {"origins": "*"}})

@app.route('/summarise', methods=['POST'])
def summarise():
    data = {}

    try:
        data = request.get_json()
        print("JSON conversion worked")
    except Exception as e:
        print("Could not decode JSON data: ", e)
    
    
    if not data or not isinstance(data, dict):
        return jsonify({'error': 'Invalid JSON data: expected a dictionary'}), 400

    # Initialize tokenizer and model (to be loaded only once)
    tokenizer = AutoTokenizer.from_pretrained(MODEL)
    tokenizer.pad_token = tokenizer.eos_token
    model = AutoModelForCausalLM.from_pretrained(MODEL)
    model.resize_token_embeddings(len(tokenizer))

    # Use Metal Performance Shaders (MPS) for Apple M1 chip, fallback to CPU if unavailable
    # TO DO: simple to add CUDA & other support.
    device = torch.device("mps" if torch.backends.mps.is_available() else "cpu")
    model.to(device)

    # Prepare the batch of texts and their corresponding document_ids
    document_ids = list(data.keys())  # Get the list of document IDs
    texts_to_process = list(data.values())  # Get the list of document texts


    # Tokenize the batch of texts
    inputs = tokenizer(texts_to_process, padding=True, truncation=True, return_tensors="pt", max_length=512)
    
    # Move the tokenized inputs to the device (GPU or CPU)
    inputs = {key: value.to(device) for key, value in inputs.items()}

    # Generate summaries for the batch of documents
    with torch.no_grad():
        summary_ids = model.generate(
            inputs['input_ids'],
            max_length=150,  # Limit the summary length
            num_beams=5,  # Beam search for better quality
            no_repeat_ngram_size=2,  # Prevent repeating n-grams
            temperature=0.7,  # Lower for deterministic output
            top_p=0.9,  # Nucleus sampling
            top_k=50,  # Consider top k tokens for diversity
        )
    
    summaries = {}

    # Decode the generated summaries and map them to the corresponding document IDs
    for idx, summary_id in enumerate(summary_ids):
        summary_text = tokenizer.decode(summary_id, skip_special_tokens=True)
        document_id = document_ids[idx]  
        summaries[document_id] = summary_text

    # Return the map of document_id -> summary as a JSON response
    return jsonify(summaries)

    
@app.route('/search', methods=['GET'])
def search():
    query = request.args.get('query')
    if not query:
        return jsonify({'error': 'No query provided'}), 400

    params = {
        "q": query,
        "key": API_KEY,
        "cx": PROGRAMMABLE_ENGINE_KEY,
        "num": 10
    }

    response = requests.get(SEARCH_URL, params=params)

    if response.status_code != 200:
        return jsonify({'error': 'Failed to fetch search results'}), response.status_code

    results = []
    for item in response.json().get('items', []):
        result = {
            "title": item.get("title"),
            "url": item.get("link"),
            "description": item.get("snippet"),
            "engine": "Google"
        }
        results.append(result)

    return jsonify(results)



if __name__ == '__main__':
    app.run(debug=True)
