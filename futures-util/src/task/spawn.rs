use futures_core::task::{LocalSpawn, Spawn};

if_std! {
    use crate::future::{FutureExt, RemoteHandle};
    use futures_core::future::{Future, FutureObj, LocalFutureObj};
    use futures_core::task::SpawnError;
}

impl<Sp: ?Sized> SpawnExt for Sp where Sp: Spawn {}
impl<Sp: ?Sized> LocalSpawnExt for Sp where Sp: LocalSpawn {}

/// Extension trait for `Spawn`.
pub trait SpawnExt: Spawn {
    /// Spawns a task that polls the given future with output `()` to
    /// completion.
    ///
    /// This method returns a [`Result`] that contains a [`SpawnError`] if
    /// spawning fails.
    ///
    /// You can use [`spawn_with_handle`](SpawnExt::spawn_with_handle) if
    /// you want to spawn a future with output other than `()` or if you want
    /// to be able to await its completion.
    ///
    /// Note this method will eventually be replaced with the upcoming
    /// `Spawn::spawn` method which will take a `dyn Future` as input.
    /// Technical limitations prevent `Spawn::spawn` from being implemented
    /// today. Feel free to use this method in the meantime.
    ///
    /// ```
    /// #![feature(async_await, await_macro, futures_api)]
    /// use futures::executor::ThreadPool;
    /// use futures::task::SpawnExt;
    ///
    /// let mut executor = ThreadPool::new().unwrap();
    ///
    /// let future = async { /* ... */ };
    /// executor.spawn(future).unwrap();
    /// ```
    #[cfg(feature = "std")]
    fn spawn<Fut>(&mut self, future: Fut) -> Result<(), SpawnError>
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.spawn_obj(FutureObj::new(Box::new(future)))
    }

    /// Spawns a task that polls the given future to completion and returns a
    /// future that resolves to the spawned future's output.
    ///
    /// This method returns a [`Result`] that contains a [`RemoteHandle`], or, if
    /// spawning fails, a [`SpawnError`]. [`RemoteHandle`] is a future that
    /// resolves to the output of the spawned future.
    ///
    /// ```
    /// #![feature(async_await, await_macro, futures_api)]
    /// use futures::executor::ThreadPool;
    /// use futures::future;
    /// use futures::task::SpawnExt;
    ///
    /// let mut executor = ThreadPool::new().unwrap();
    ///
    /// let future = future::ready(1);
    /// let join_handle_fut = executor.spawn_with_handle(future).unwrap();
    /// assert_eq!(executor.run(join_handle_fut), 1);
    /// ```
    #[cfg(feature = "std")]
    fn spawn_with_handle<Fut>(
        &mut self,
        future: Fut
    ) -> Result<RemoteHandle<Fut::Output>, SpawnError>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send,
    {
        let (future, handle) = future.remote_handle();
        self.spawn(future)?;
        Ok(handle)
    }
}

/// Extension trait for `LocalSpawn`.
pub trait LocalSpawnExt: LocalSpawn {
    /// Spawns a task that polls the given future with output `()` to
    /// completion.
    ///
    /// This method returns a [`Result`] that contains a [`SpawnError`] if
    /// spawning fails.
    ///
    /// You can use [`spawn_with_handle`](SpawnExt::spawn_with_handle) if
    /// you want to spawn a future with output other than `()` or if you want
    /// to be able to await its completion.
    ///
    /// Note this method will eventually be replaced with the upcoming
    /// `Spawn::spawn` method which will take a `dyn Future` as input.
    /// Technical limitations prevent `Spawn::spawn` from being implemented
    /// today. Feel free to use this method in the meantime.
    ///
    /// ```
    /// #![feature(async_await, await_macro, futures_api)]
    /// use futures::executor::LocalPool;
    /// use futures::task::LocalSpawnExt;
    ///
    /// let executor = LocalPool::new();
    /// let mut spawner = executor.spawner();
    ///
    /// let future = async { /* ... */ };
    /// spawner.spawn_local(future).unwrap();
    /// ```
    #[cfg(feature = "std")]
    fn spawn_local<Fut>(&mut self, future: Fut) -> Result<(), SpawnError>
    where
        Fut: Future<Output = ()> + 'static,
    {
        self.spawn_local_obj(LocalFutureObj::new(Box::new(future)))
    }

    /// Spawns a task that polls the given future to completion and returns a
    /// future that resolves to the spawned future's output.
    ///
    /// This method returns a [`Result`] that contains a [`RemoteHandle`], or, if
    /// spawning fails, a [`SpawnError`]. [`RemoteHandle`] is a future that
    /// resolves to the output of the spawned future.
    ///
    /// ```
    /// #![feature(async_await, await_macro, futures_api)]
    /// use futures::executor::LocalPool;
    /// use futures::future;
    /// use futures::task::LocalSpawnExt;
    ///
    /// let mut executor = LocalPool::new();
    /// let mut spawner = executor.spawner();
    ///
    /// let future = future::ready(1);
    /// let join_handle_fut = spawner.spawn_local_with_handle(future).unwrap();
    /// assert_eq!(executor.run_until(join_handle_fut), 1);
    /// ```
    #[cfg(feature = "std")]
    fn spawn_local_with_handle<Fut>(
        &mut self,
        future: Fut
    ) -> Result<RemoteHandle<Fut::Output>, SpawnError>
    where
        Fut: Future + 'static,
    {
        let (future, handle) = future.remote_handle();
        self.spawn_local(future)?;
        Ok(handle)
    }
}
