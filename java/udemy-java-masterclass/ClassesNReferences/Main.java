public class Main {
    public static void main (String[] args) {
	    House blueHouse = new House("blue");
	    House anotherHouse = blueHouse; // this is setting another house as a reference
					    // to the same object as blueHouse

	    System.out.println(blueHouse.getColor()); // blue
	    System.out.println(anotherHouse.getColor()); // blue

	    anotherHouse.setColor("red");
	    System.out.println(blueHouse.getColor()); // red
	    System.out.println(anotherHouse.getColor()); // red

	    House greenHouse = new House("green");
	    anotherHouse = greenHouse;

	    System.out.println(blueHouse.getColor()); // red
	    System.out.println(greenHouse.getColor()); // green
	    System.out.println(anotherHouse.getColor()); // green
    }
}
