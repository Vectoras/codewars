def sum_of_intervals(intervals):     
    result = [];
    
    for interval in intervals:
        for k in range (interval[0], interval[1]):
            if k not in result:
                result.append(k);        
        
    return len(result);