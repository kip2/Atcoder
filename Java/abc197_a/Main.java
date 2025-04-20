import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        String input = getLineAsString();
        System.out.println(solve(input));
    }

    public static String getLineAsString() {
        Scanner sc = new Scanner(System.in);
        String line = sc.nextLine();
        sc.close();
        return line;
    }

    public static String solve(String input) {
      String first = input.substring(0, 1);
      String rest = input.substring(1);
      return rest + first;
    }
}



