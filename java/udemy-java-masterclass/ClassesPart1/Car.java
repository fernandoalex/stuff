public class Car {

	private String make = "Tesla";
	public String getMake () {
		return make;
	}

	public void setMake (String make) {
		this.make = make;
	}

	private String model = "Model X";
	public String getModel () {
		return model;
	}
	public void setModel (String model) {
		this.model = model;
	}

	private String color = "Gray" ;
	public String getColor () {
		return color;
	}

	public void setColor (String color) {
		this.color = color;
	}

	private int doors = 2;
	public int getDoors () {
		return doors;
	}

	public void setDoors (int doors) {
		this.doors = doors;
	}

	private boolean convertible = true;
	public boolean getConvertible () {
		return convertible;
	}

	public void getConvertible (boolean convertible) {
		this.convertible = convertible;
	}

	public Car () {
		System.out.println("constructor called");

	}

	public Car (String make, String model, String color, int doors, boolean convertible) {
		System.out.println("constructor with args");
		this.make = make;
		this.model = model;
		this.color = color;
		this.doors = doors;
		this.convertible = convertible;
	}

	public void describeCar() {

		System.out.println ( 
			 doors + "-Doors " +
			 color + " " +
			 make + " " +
			 model + " " +
			 (convertible ? "Convertible" : "")
		);
	}
}
