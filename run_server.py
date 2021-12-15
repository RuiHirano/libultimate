import argparse
from libultimate.server import UltimateServer

parser = argparse.ArgumentParser(description='')
parser.add_argument('--host')
parser.add_argument('--port')

args = parser.parse_args()
print("Running Ultimate Server at {}:{}".format(args.host, args.port))
server = UltimateServer(host=str(args.host), port=int(args.port))
server.run()