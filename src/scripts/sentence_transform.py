from sentence_transformers import SentenceTransformer
import sys
import json

# Currently assuming all relevant error codes are zero.
model = None

try:
    model = SentenceTransformer("all-MiniLM-L6-v2")

except Exception as e:
    print(json.dumps(1))
    sys.exit(1)

# Obtain sentences to decode. Currently will take a single sentence to embed as a collection of terms.
# Revise this use and use of standard Word2Vec embedding.
if len(sys.argv) < 2:
    print("Usage: python3 sentence_transform.py <terms>")
    sys.exit(1)

terms = sys.argv[1:]
sentences = [" ".join(terms)]
embeddings_compl = []

# Calculate model embeddings
try:
    if model is not None:
        embeddings = model.encode(sentences)
        for embedding in embeddings:
            embeddings_compl.append(embedding.tolist())
            
        print(json.dumps(embeddings_compl))
        sys.exit(0)

except Exception as e:
    print("Obtained error:", e)
    print(json.dumps(0))
    sys.exit(1)

print(json.dumps(0))
sys.exit(1)

# (Optional) return model similarities (currently we have existing implementation for Cosine & Minkowski distances in Rust).

