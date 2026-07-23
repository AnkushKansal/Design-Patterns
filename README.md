## Implemented :

Creational:
1) Singleton : Acess to single Instance of type via static and keeping it thread safe with Mutex.
Remember, Mutex has interior mutability, so multiple can aceess and modify the inner values of a type in thread safe manner
Therefore, We need not to make the static global instance mutable and saves us the nusance of unsafe code

2) Abstact Factory : Abstract factory pattern with example of multiple payment provider
Payment providers are thread accessible via std::marker traits

Structural
3) Facade : A client application makes/interface with facade type which hides all the internal api calling for several client provide API