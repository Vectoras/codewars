def spin_words(sentence):
    wArr = sentence.split()
    results = []
    for item in wArr:
        if len(item) >= 5:
            results.append(item[::-1])
        else:
            results.append(item)
    return ' '.join(results)