# RECURSILVE SOLUTION WITH DETAILED COMMENTS

def get_pins(observed):
    return get_possible_combos(list(observed));

def get_possible_combos(pin):    
    
    results = []
    receivedResults = []
    posibilities = {
            '1' : ['1', '2', '4'],
            '2' : ['1', '2', '3', '5'],
            '3' : ['2', '3', '6'],
            '4' : ['1', '4', '5', '7'],
            '5' : ['2', '4', '5', '6', '8'],
            '6' : ['3', '5', '6', '9'],
            '7' : ['4', '7', '8'],
            '8' : ['5', '7', '8', '9', '0'],
            '9' : ['6', '8', '9'],
            '0' : ['8', '0']
            }    
    
    # extract current PIN digit
    digit = pin.pop(0);
    
    # if at the end of initial PIN, meaning last digit -> all options are the ones in posibilities
    if len(pin) == 0:
        return posibilities[digit];
    
    # if it passes the above test, meaning not at the end of pin -> 
    
    # get all combinations based on the rest of the digits without this one (eg: 369 -> 69)
    receivedResults = get_possible_combos(pin.copy());
    
    # go over all the possible digits 
    for p in posibilities[digit]:
        # add current possible digit to all the possible combinations recevied from the next step
        for r in receivedResults:
            results.append (p + r);
        
    return results;




# RECURSIVE SOLUTION WITHOUT COMMENTS

def get_pins(observed):
    return get_possible_combos(list(observed));

def get_possible_combos(pin):    
    
    results = []
    receivedResults = []
    posibilities = {
            '1' : ['1', '2', '4'],
            '2' : ['1', '2', '3', '5'],
            '3' : ['2', '3', '6'],
            '4' : ['1', '4', '5', '7'],
            '5' : ['2', '4', '5', '6', '8'],
            '6' : ['3', '5', '6', '9'],
            '7' : ['4', '7', '8'],
            '8' : ['5', '7', '8', '9', '0'],
            '9' : ['6', '8', '9'],
            '0' : ['8', '0']
            }    
    
    digit = pin.pop(0);
    if len(pin) == 0:
        return posibilities[digit];
    receivedResults = get_possible_combos(pin.copy());
    for p in posibilities[digit]:
        for r in receivedResults:
            results.append (p + r);
        
    return results;