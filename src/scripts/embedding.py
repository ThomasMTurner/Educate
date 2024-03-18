import sys
import json
import os
from gensim.models import KeyedVectors


if len(sys.argv) != 2:
    print("Usage: python3 embedding.py <term>")
    sys.exit(1)

TERM = sys.argv[1]


model = None
loaded_saved_model = False

try:  
    # Load the model in text format
    if not os.path.exists('./scripts/models/model.bin'):
        model = KeyedVectors.load_word2vec_format('./scripts/models/Word2VecModel.vec', binary=False)

    else:
        loaded_saved_model = True
        model = KeyedVectors.load_word2vec_format('./scripts/models/model.bin', binary=True)


except Exception as e:
    print("Obtained the following error trying to load the model: ", e)


word_embedding = None

if model is None:
    print("Cannot make embedding - failed to load model")

else:
    ## Save the model in binary format.
    if not loaded_saved_model:
        model.save_word2vec_format('./scripts/models/model.bin', binary=True)
    word_embedding = model[TERM]

if word_embedding is None:
    print("Failed to make embedding")

else:
    word_embedding = word_embedding.tolist()
    print(json.dumps(word_embedding))
