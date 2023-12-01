import random

types = ["field", "integer", "boolean", "string", "array", "tuple"]

# Generate a random type from a list of types
def rtype():
    return random.choice(types)

# Generate a random field
def rfield():
    return rint(0, 21888242871839275222246405745257275088548364400416034343698204186575808495617)

# Generate a random integer between the bounds
def rint(min, max):
    if min > max:
        return random.randint(max, min)
    return random.randint(min, max)

# Generate a random boolean
def rbool():
    return random.choice([True, False])

# Generate a random string
def rstr(min, max):
    length = rint(min, max)
    string = ""
    for i in range(length):
        string += chr(random.randint(65, 122))
    return string

# Generate an array of one type using the given random generator
def rarray_of_type(rgenerator, size):
    array = []
    for i in range(size):
        array.append(rgenerator())
    return array

# Generate an array of a random type
def rtype_array(size, types):
    return rarray_of_type(lambda: rtype(types), size)

# Generate an array of random value using a random type 
def rvaluearray(size, types):
    return rtypearray(size, lambda: rvalue(types))

rtypearray(size, [rfield])