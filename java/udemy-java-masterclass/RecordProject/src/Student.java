// class Student {
//
//     private String id;
//     private String name;
//     private String dataOfBirth;
//     private String classList;
//
//     public Student (String id, String name, String dataOfBirth, String classList) {
// 	    this.id = id;
// 	    this.name = name;
// 	    this.dataOfBirth = dataOfBirth;
// 	    this.classList = classList;
//     }
//
// }
// Records type
public record Student (String id, String name, String dataOfBirth, String classList) {

}
