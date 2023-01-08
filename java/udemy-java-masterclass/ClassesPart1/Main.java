public class Main {
	public static void main (String[] args) {
		Car car = new Car();
		car.describeCar();

		car.setColor("red");
		car.describeCar();


		// this will cause a runtime error, not a compile error
		// Car car1 = null;
		// car1.describeCar();
		//
		Car gol = new Car("gol","black", "2001", 4, true);
		gol.describeCar();
	}
}
