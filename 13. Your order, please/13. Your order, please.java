// 13. Your order, please
// https://www.codewars.com/kata/55c45be3b2079eccff00010f/

public class Order {
  public static String order(String words) {
    
    if(words == "")
      return "";
    
    String[] wordsArray = words.split(" ");
    String[] results = new String[wordsArray.length];
    
    for (int i = 0; i < wordsArray.length; i++) {
        int location = Integer.parseInt(wordsArray[i].replaceAll("\\D+",""));
        results[location-1] = wordsArray[i];
    }
    
  return String.join(" ", results);
  }
}