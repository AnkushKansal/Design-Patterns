## Implemented :

1) Singleton : Acess to single Instance of type via static and keeping it thread safe with Mutex.
Remember, Mutex has interior mutability, so multiple can aceess and modify the inner values of a type in thread safe manner
Therefore, We need not to make the static global instance mutable and saves us the nusance of unsafe code

