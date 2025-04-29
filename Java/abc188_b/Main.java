import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
      Scanner sc = new Scanner(System.in);
      int n = getLineAsInt(sc);
      int[] a = getLineAsIntArray(sc);
      int[] b = getLineAsIntArray(sc);
      sc.close();

      String result = solve(n, a, b);
      System.out.println(result);
    }

    public static String getLineAsString(Scanner sc) {
      return sc.nextLine();
    }

    public static int getLineAsInt(Scanner sc) {
      return Integer.parseInt(sc.nextLine());
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

    public static String solve(int n, int[] a, int[] b) {
      int sum = 0;
      for (int i = 0; i < n; i++) {
        sum += a[i] * b[i];
      }

      if (sum == 0) {
        return "Yes";
      } else {
        return "No";
      }
    }
}


