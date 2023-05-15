from translator import *
import sys

def usage():
    print("\nUSAGE:\n    --encode <text>\n    --decode <text>\n")
    sys.exit(1)

def main():
    try:
        mode = sys.argv[1]
        match mode:
            case "--encode":
                try:
                    print(translate(sys.argv[2]))
                except IndexError:
                    usage()
            case "--decode":
                try:
                    print(decode(sys.argv[2]))
                except IndexError:
                    usage()
            case _:
                usage()
    except IndexError:
        print("\nUSAGE:\n    --encode <text>\n    --decode <text>\n")

if __name__ == "__main__":
    main()
