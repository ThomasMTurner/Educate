import sys
import json
import os
from gensim.models import KeyedVectors

if len(sys.argv) < 2:
    print("Usage: python3 embedding.py <terms>")
    sys.exit(1)

TERMS = sys.argv[1:]
print("Obtained terms: ", TERMS)

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
    print(json.dumps(0))

embeddings = []
errors = 0

if model is None:
    print(json.dumps(0))

else:
    ## Save the model in binary format.
    if not loaded_saved_model:
        model.save_word2vec_format('./scripts/models/model.bin', binary=True)

    # TO DO: gracefully carry the KeyError warning until all TERMS have been enumerated.
    for TERM in TERMS:
        try:
            embedding = model[TERM]
            if embedding is not None:
                embeddings.append(embedding.tolist())
            else:
                continue
        except KeyError:
            # Assuming error code 1 represent (term not found in model vocabulary).
            print(json.dumps(1))
            sys.exit(1)



print(json.dumps(embeddings))
