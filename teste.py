from target.release import librust
import json

teste = {"testing": {"this is a JSON test": 893}, "stillTesting": "aklsdjaslk"}

data = librust.read(json.dumps(teste))

print(json.loads(data)["this is a JSON test"])