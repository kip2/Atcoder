import java.util.Arrays;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
      Scanner sc = new Scanner(System.in);
      int[] line1 = getLineAsIntArray(sc);
      int[] a = getLineAsIntArray(sc);

      int _n = line1[0];
      int x = line1[1];

      List<Integer> result = solve(x, a);
      String output = joinListInteger(result);

      System.out.println(output);

      sc.close();
    }

    public static List<Integer> solve(int x, int[] a) {
      List<Integer> list = convIntArrayToList(a);

      return list.stream()
          .filter(n -> n != x)
          .collect(Collectors.toList());
    }

    public static List<Integer> convIntArrayToList(int[] list) {
      return Arrays.stream(list)
                    .boxed()
                    .collect(Collectors.toList());
    }

    public static String joinListInteger(List<Integer> list) {
      return list.stream() 
                  .map(String::valueOf)
                  .collect(Collectors.joining(" "));
    }

    public static String getLineAsString(Scanner sc) {
        String line = sc.nextLine();
        return line;
    }

    public static int[] getLineAsIntArray(Scanner sc) {
      String line = sc.nextLine();

      String[] tokens = line.trim().split("\\s+");
      int[] nums = new int[tokens.length];
      for (int i = 0; i < tokens.length; i++) {
        nums[i] = Integer.parseInt(tokens[i]);
      }
      return nums;
    }
}


