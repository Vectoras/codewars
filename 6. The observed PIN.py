def get_pins(observed):
    array = list(observed)
#     obv_array = list(observed)
#     return get_possible_combos(list(observed))
#     print("this is input: ", observed)
#     ('11',["11", "22", "44", "12", "21", "14", "41", "24", "42"])
#       loop the the integers of the number "11"
#     print(observed)
#     print(obv_array[len(obv_array)-1])
#     return []
    pos_combo = {
            1 : [1, 2, 4],
            2 : [1, 2, 3, 5],
            3 : [2, 3, 6],
            4 : [1, 4, 5, 7],
            5 : [2, 4, 5, 6, 8],
            6 : [3, 5, 6, 9],
            7 : [4, 7, 8],
            8 : [5, 7, 8, 9, 0],
            9 : [6, 8, 9],
            0 : [8, 0]
            }
        
#     for element in pos_combo[array(0)]:
#         print(element)
    pos_combo[int(array[0])]
    array_digit = array.pop(0)
    print("this is digit: ", array_digit)
    print("this is array: ", array)
    print("this is given: ", observed)
    return []

def get_possible_combos(array):
    results = []
    pos_combo = {
            1 : [1, 2, 4],
            2 : [1, 2, 3, 5],
            3 : [2, 3, 6],
            4 : [1, 4, 5, 7],
            5 : [2, 4, 5, 6, 8],
            6 : [3, 5, 6, 9],
            7 : [4, 7, 8],
            8 : [5, 7, 8, 9, 0],
            9 : [6, 8, 9],
            0 : [8, 0]
            }
    array_digit = array.pop(0)
    for element in pos_combo[array_digit]:
        if len(array) == 0:
            return "STAP"
        results = get_possible_combos(array)
        results.append(element)
        
    return results