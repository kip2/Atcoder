import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // case: input to int[]
        // int[] inputs = getLineAsIntArray();
        // System.out.println("input value: " + Arrays.toString(inputs));

        // case: input to string
        // String input = getLineAsString();
        // System.out.println("input value: " + input);
    }

    public static String getLineAsString() {
        Scanner sc = new Scanner(System.in);
        String line = sc.nextLine();
        sc.close();
        return line;
    }

    public static int[] getLineAsIntArray() {
      Scanner sc = new Scanner(System.in);
      String line = sc.nextLine();
      sc.close();

      String[] tokens = line.trim().split("\\s+");
      int[] nums = new int[tokens.length];
      for (int i = 0; i < tokens.length; i++) {
        nums[i] = Integer.parseInt(tokens[i]);
      }
      return nums;
    }

    public static String solve() {
      return null;
    }
}


