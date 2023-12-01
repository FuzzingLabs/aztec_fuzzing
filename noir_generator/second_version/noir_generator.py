import sys
import experimental_generator
import classic_generator

def main():
    if len(sys.argv) == 2 and (sys.argv[1] == "-experimental" or sys.argv[1] == "-e"):
        experimental_generator.generate()
        return
    classic_generator.generate()

if __name__ == "__main__":
    main()
