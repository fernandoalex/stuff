public class Main {

    public static void main (String[] args) {

	    int myFirstNumber = 12;
	    int mySecondNumber = 12;

	    int myTotal = myFirstNumber + mySecondNumber;

	    System.out.println(myTotal);
	    int myMinIntValue = Integer.MIN_VALUE;
	    int myMaxIntValue = Integer.MAX_VALUE;

	    System.out.println(myMinIntValue);
	    System.out.println(myMaxIntValue);

	    // Valid way to type numbers
	    int testInt1 = 100_000; // this is 100000
	    System.out.println(testInt1);

	    byte myMinByteValue = Byte.MIN_VALUE;
	    byte myMaxByteValue = Byte.MAX_VALUE;

	    System.out.println(myMinByteValue);
	    System.out.println(myMaxByteValue);
	
	    short myMinShortValue = Short.MIN_VALUE;
	    short myMaxShortValue = Short.MAX_VALUE;

	    System.out.println(myMinShortValue);
	    System.out.println(myMaxShortValue);

	    long myLongValue = 100L;
	    long myMinLongValue = Long.MIN_VALUE;
	    long myMaxLongValue = Long.MAX_VALUE;

	    System.out.println(myMinLongValue);
	    System.out.println(myMaxLongValue);
	    System.out.println(myLongValue);

	    byte myNewByteValue = (byte)(myMinByteValue / 2);
	    System.out.println(myNewByteValue);
	    
    }
}
