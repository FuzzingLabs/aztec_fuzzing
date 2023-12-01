import random

basic_operations = ["+", "-", "*", "/"]#, "%"] Not yet supported by nargo compiler

supported_types = ["Field"]
supported_operations = [basic_operations]#, "assertion", "if", "..."]

def generate_random_number(min_bound, max_bound):
    if min_bound > max_bound:
        return random.randint(max_bound, min_bound)
    return random.randint(min_bound, max_bound)

def generate_boolean():
    return random.choice([True, False])

def generate_field(field):
    return "Field::new(" + str(field) + ")"
    pass

def generate_basic_operation(arg_nb, operation, code, used_variables, i):
    operation = random.choice(basic_operations)
    arg1 = generate_random_number(0, arg_nb-1)
    arg2 = generate_random_number(0, arg_nb-1)

    ## Only in experimental mode because not yet supported by nargo compiler
    # shorthand = generate_boolean()
    # if shorthand:
    #     code[0] += "\tlet arg" + str(arg1) + " " + operation + "= arg" + str(arg2) + ";\n"
    #     used_variables.append("arg" + str(arg1))
    #     pass
    code[0] += "\tlet result" + str(i) + " = " + "arg" + str(arg1) + " " + operation + " arg" + str(arg2) + ";\n"
    used_variables.append("result" + str(i))

def generate_return(used_variables, code, arg_nb):
    if not used_variables:
        code[0] += "\targ" + str(generate_random_number(0, arg_nb)) + ";\n}\n"
    else:
        code[0] += "\t" + random.choice(used_variables) + ";\n}\n"

def generate_code(code, arg_nb):
    used_variables = []
    
    operations_nb = generate_random_number(3, 10)
    for i in range(operations_nb):

        operation = random.choice(supported_operations)

        if operation == basic_operations:
            generate_basic_operation(arg_nb, operation, code, used_variables, i)

        elif operation == "assertion":
            arg1 = generate_random_number(0, arg_nb)
            code[0] += "\tassert!(...);\n"

    generate_return(used_variables, code, arg_nb)


def generate_argument(code, min_bound, max_bound):
    arg_nb = generate_random_number(min_bound, max_bound)
    for i in range(arg_nb):
        if i != 0:
            code[0] += ", "
        code[0] += "arg" + str(i) + ": " + random.choice(supported_types)
    code[0] += ")"
    return arg_nb

def generate_return_type(code):
    code[0] += " -> " + random.choice(supported_types) + "\n{\n"

def generate_main():
    code = ["\nfn main("]

    arg_nb = generate_argument(code, 2, 10)
    generate_return_type(code)
    generate_code(code, arg_nb)

    return code[0]

def main():
    print(generate_main())

if __name__ == "__main__":
    main()

