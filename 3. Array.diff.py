def array_diff(a, b):
    result = a;
    for i in b:
        result = list(filter(lambda j: j != i, result))
        print (result);
    print ('result: ', result)
    return result;