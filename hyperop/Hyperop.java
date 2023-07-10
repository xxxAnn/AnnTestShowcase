public class Hyperop {
    public static void main(String[] args) {
        System.out.println(String.format("%s", hyperoperation(3, 4, 3)));
    }

    public static long hyperoperation(long a, int n, long b) {
        if (n == 3) {
            return (long) Math.pow(a, b);
        }
        long prev = a;
        int subn = n - 1;
        for (int i = 0; i < b-1; i++) {
            prev = hyperoperation(a, subn, prev);
        }

        return prev;
    }
}