from collections import Counter;

def anagrams(word, words):   
    return [w for w in words if Counter(list(w)) == Counter(list(word))]