/* 16. Split Strings
https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/java */

public class StringSplit {
    public static String[] solution(String s) {
      if (s.length() % 2 == 1) {
        s += "_";
      }
      String[] results = new String[s.length()/2];
      for (int k = 0, i = 0; i < s.length(); i+=2, k++) {
          String working_string = "";
          working_string = "" + s.charAt(i) + s.charAt(i+1);
          results[k] = working_string;
      }
      return results;
    }
}