import argparse

parser = argparse.ArgumentParser(description='This is a simple script to show argparser')

parser.add_argument('--version', action='version', version='%(prog)s 0.1')
parser.add_argument('--arg1', dest='arg1', action='store',
                   help='The frist argument')
parser.add_argument('--req1', dest='req1', action='store',
                   help='The frist req argument', required=True)

args = parser.parse_args()

if __name__ == '__main__':
        print(args.req1)
        print(args.arg1)