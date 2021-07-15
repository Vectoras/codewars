// 10. Complementary DNA
// https://www.codewars.com/kata/554e4a2f232cdd87d9000038/

// SOLUTION 1
public class DnaStrand {
  public static String makeComplement(String dna) {
    
    dna = dna.replace('A','1');
    dna = dna.replace('T','2');
    dna = dna.replace('C','3');
    dna = dna.replace('G','4');
    
    dna = dna.replace('1','T');
    dna = dna.replace('2','A');
    dna = dna.replace('3','G');
    dna = dna.replace('4','C');
    
    return dna;
  }
}


// SOLUTION 2
public class DnaStrand {
  public static String makeComplement(String dna) {
    
    String result = "";       
    
    for (int i = 0; i < dna.length(); i++) {
      switch (dna.charAt(i)) {
          case 'A':
            result += 'T';
            break;
          case 'T':
            result += 'A';
            break;
          case 'G':
            result += 'C';
            break;
          case 'C':
            result += 'G';
            break;
      }
    }
    
    
    return result;
  }
}
