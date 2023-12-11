# Common
------------------------------------------------------------------------
This file describes the Specifications of what the Common Crate should contain and implement.
Recall that the WG will have to provide one implementation for the Common Crate for all groups to use, and that implementation must adhere to these Specifications.

## World mod
------------------------------------------------------------------------
The World module should export these things:
- ```enum TileType```:
    - contains the variants like Grass, Mountain, Street, Water...
- ```struct Tile```:
    - contains the TileType for a particular cell with additional information / metadata (is on fire, contains water)
- ```struct World```:
    - must be public
    - contains the 2d map(```Vec<Vec<Tile>>```), the dimension of the ```World``` (rows, columns), the coordinates of the robot.
- ```struct PrivateWorld```:
    - must be private
    - contains the same information of ```World```, but in addition to that there should be some data that are not decided by the world (how many interfaces are left for that particular robot).
- ```trait WorldGenerator```:
    - contains a method ```new(&mut self)->World``` which returns the new ```World```
    - must be public

## Energy mod
------------------------------------------------------------------------
The Energy module should export these things:
- ```Energy```:
    - it contains a field ```quantity usize```, for how much energy is holding.
    - it must impl Default (with 0 quantity)
    - it contains a ```new(quantity: usize)->Self``` function which creates a new Energy struct, setting the quantity field to the input. This function should be private to this crate because we do not want to allow free energy creation.
    - ```fn get(&mut self, quantity: usize)->Result<Self, ()>``` splits the energy in two struct, and return the corresponding quantity (if enough energy is available), or return an Error.
    - it must implement the ```AddAssign``` trait (so we could join two energy struct together)

NOTES:
The ```new``` specification is correct. It defines the signature correctly, and describes the expected behaviour correctly.
You should strive to make all the contents of this file this precise.
On the other hand, the ```get``` specification is not correct. Its description says to split the energy in two structs, but it only returns Self. Where is the other Energy? What kind of Error should be returned? 
An Error of Void is like an Option, so either use an option (not the right answer) or specify the error type, perhaps with a String, or perhaps with an enum of Error choices, so that the client of this function receives useful information.
The ```AddAssign``` trait specification is also underspecified. How does one join two Energies? Via which method or function?
More importantly, should this really be a Trait, or just another method in the Energy struct?

## Runner mod
------------------------------------------------------------------------
The Runner module should export these things:
- ```Robot trait``` is a Trait that contains a single method ```fn process_tick(&mut self, energy: &Energy)```, which takes 100 Energy generated from the runner
- ```runner``` this function must create 100 Energy and pass it to the robot through ```process_tick(&mut robot, energy: Energy)```, looping for a fixed amount of times

NOTES:
The choice of the value 100 here is arbitrary. Avoid getting bogged down in details such as knowing how much energy to supply to the robot before moving on. 
This is a detail you can easily fix in no time once you have defined the Interfaces.

## Interface mod
------------------------------------------------------------------------
The Interface mod contains all the interfaces that the Tools can use, essentially describing the basic building blocks for creating Tools that read the world and Tools that change the world.
- ```BasicInterface```: has a mutable reference to ```PrivateWorld```, and has to be cloned at most n-1 times, where n is saved in world. it should only implement a ```From<World>``` 
- Every Other interface: They should impl the ```From<BasicInterface>``` trait. They should contain a mutable reference to the world as well as additional functions needed for this particular type of interfaces. Some examples are listed below:
- ```Move``` interface:
    - contains only an ```Rc<Mutex<PrivateWorld>>```
    - implements ```From<BasicInterface>```
    - has a method ```fn move_me(&mut self, energy: &mut Energy, direction: Directions)->Result<(), ()>```, it must consume 10 of energy (or return ```Err```), then return ```Ok``` if the movement succeeded (no out of bound, no obstacle in the arrival Cell...), and obviously modify the x, y coordinates of the Robot in ```PublicWorld```. 
- ```Destroy``` interface:
    - contains only an ```Rc<RefCell<Box<Mutex<Arc<PrivateWorld>>>>>```
    - implements ```From<BasicInterface>```
    - has a method ```fn destroy(&mut self, energy: &mut Energy, direction: Directions)->Result<(), Box<String>>```, it must consume 500 of energy (or return ```Err```), then destroy the content of the corresponding cell(if there is something to destroy) e.g., mountain in grass, street in grass, nothing on grass...
- ```Debug``` interface:
    - must be compiled only when ```cfg(test)``` (when executing tests).
    - has a method ```fn print_world(&mut self)``` which should print out on the terminal the content of the PrivateWorld and all the information according the robot. You should use this interface when debugging the behaviour of your Tools

### Interface Constraints
------------------------------------------------------------------------
Reasonable constraints.
It should not be possible (besides from the Debug Interface) to, for example, visualise the whole world, or burn all of it. Or at least, Interfaces should not allow it. 
Tools, on the other side (but recall that Tools are not defined in the Commons Crate), could do those things, for example by storing enough energy and using it to, for example sense as much world as possible.

NOTES:
keep the functionalities of Interfaces to a minimum