import numpy as np
import sys
import json
from sklearn.cluster import KMeans

if __name__ == "__main__":
    # Obtain the document embedding from command line argument.
    if len(sys.argv) != 2:
        print(json.dumps("Usage python3 cluster.py <document_embedding_values>"))
        sys.exit(1)

    # Deserialise input samples
    samples = json.loads(sys.argv[1])

    # Process Kmeans.
    samples = np.array(samples)

    kmeans = KMeans(n_clusters=3, random_state=0, n_init="auto").fit(samples)
    
    if kmeans.labels_ is not None:
        labels = kmeans.labels_.astype(np.int32)
    
        out = []
        for label in np.unique(labels):
            embeddings = samples[labels == label]
            centroid = kmeans.cluster_centers_[label]
            out.append((centroid.tolist(), embeddings.tolist()))

        # Serialise output - as JSON dictionary.
        try: 
            print(json.dumps(list(out)).strip())

        except Exception as e:
            print(json.dumps(e))

    else:
        print(json.dumps("No labels found."))


    
