/* automatically generated by rust-bindgen */

# [ doc = " CUDA device properties" ] # [ repr ( C ) ] pub struct cudaDeviceProp { # [ doc = "< ASCII string identifying device" ] pub name : [ :: std :: os :: raw :: c_char ; 256usize ] , # [ doc = "< Global memory available on device in bytes" ] pub totalGlobalMem : usize , # [ doc = "< Shared memory available per block in bytes" ] pub sharedMemPerBlock : usize , # [ doc = "< 32-bit registers available per block" ] pub regsPerBlock : :: std :: os :: raw :: c_int , # [ doc = "< Warp size in threads" ] pub warpSize : :: std :: os :: raw :: c_int , # [ doc = "< Maximum pitch in bytes allowed by memory copies" ] pub memPitch : usize , # [ doc = "< Maximum number of threads per block" ] pub maxThreadsPerBlock : :: std :: os :: raw :: c_int , # [ doc = "< Maximum size of each dimension of a block" ] pub maxThreadsDim : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum size of each dimension of a grid" ] pub maxGridSize : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Clock frequency in kilohertz" ] pub clockRate : :: std :: os :: raw :: c_int , # [ doc = "< Constant memory available on device in bytes" ] pub totalConstMem : usize , # [ doc = "< Major compute capability" ] pub major : :: std :: os :: raw :: c_int , # [ doc = "< Minor compute capability" ] pub minor : :: std :: os :: raw :: c_int , # [ doc = "< Alignment requirement for textures" ] pub textureAlignment : usize , # [ doc = "< Pitch alignment requirement for texture references bound to pitched memory" ] pub texturePitchAlignment : usize , # [ doc = "< Device can concurrently copy memory and execute a kernel. Deprecated. Use instead asyncEngineCount." ] pub deviceOverlap : :: std :: os :: raw :: c_int , # [ doc = "< Number of multiprocessors on device" ] pub multiProcessorCount : :: std :: os :: raw :: c_int , # [ doc = "< Specified whether there is a run time limit on kernels" ] pub kernelExecTimeoutEnabled : :: std :: os :: raw :: c_int , # [ doc = "< Device is integrated as opposed to discrete" ] pub integrated : :: std :: os :: raw :: c_int , # [ doc = "< Device can map host memory with cudaHostAlloc/cudaHostGetDevicePointer" ] pub canMapHostMemory : :: std :: os :: raw :: c_int , # [ doc = "< Compute mode (See ::cudaComputeMode)" ] pub computeMode : :: std :: os :: raw :: c_int , # [ doc = "< Maximum 1D texture size" ] pub maxTexture1D : :: std :: os :: raw :: c_int , # [ doc = "< Maximum 1D mipmapped texture size" ] pub maxTexture1DMipmap : :: std :: os :: raw :: c_int , # [ doc = "< Maximum size for 1D textures bound to linear memory" ] pub maxTexture1DLinear : :: std :: os :: raw :: c_int , # [ doc = "< Maximum 2D texture dimensions" ] pub maxTexture2D : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 2D mipmapped texture dimensions" ] pub maxTexture2DMipmap : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum dimensions (width, height, pitch) for 2D textures bound to pitched memory" ] pub maxTexture2DLinear : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum 2D texture dimensions if texture gather operations have to be performed" ] pub maxTexture2DGather : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 3D texture dimensions" ] pub maxTexture3D : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum alternate 3D texture dimensions" ] pub maxTexture3DAlt : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum Cubemap texture dimensions" ] pub maxTextureCubemap : :: std :: os :: raw :: c_int , # [ doc = "< Maximum 1D layered texture dimensions" ] pub maxTexture1DLayered : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 2D layered texture dimensions" ] pub maxTexture2DLayered : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum Cubemap layered texture dimensions" ] pub maxTextureCubemapLayered : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 1D surface size" ] pub maxSurface1D : :: std :: os :: raw :: c_int , # [ doc = "< Maximum 2D surface dimensions" ] pub maxSurface2D : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 3D surface dimensions" ] pub maxSurface3D : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum 1D layered surface dimensions" ] pub maxSurface1DLayered : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Maximum 2D layered surface dimensions" ] pub maxSurface2DLayered : [ :: std :: os :: raw :: c_int ; 3usize ] , # [ doc = "< Maximum Cubemap surface dimensions" ] pub maxSurfaceCubemap : :: std :: os :: raw :: c_int , # [ doc = "< Maximum Cubemap layered surface dimensions" ] pub maxSurfaceCubemapLayered : [ :: std :: os :: raw :: c_int ; 2usize ] , # [ doc = "< Alignment requirements for surfaces" ] pub surfaceAlignment : usize , # [ doc = "< Device can possibly execute multiple kernels concurrently" ] pub concurrentKernels : :: std :: os :: raw :: c_int , # [ doc = "< Device has ECC support enabled" ] pub ECCEnabled : :: std :: os :: raw :: c_int , # [ doc = "< PCI bus ID of the device" ] pub pciBusID : :: std :: os :: raw :: c_int , # [ doc = "< PCI device ID of the device" ] pub pciDeviceID : :: std :: os :: raw :: c_int , # [ doc = "< PCI domain ID of the device" ] pub pciDomainID : :: std :: os :: raw :: c_int , # [ doc = "< 1 if device is a Tesla device using TCC driver, 0 otherwise" ] pub tccDriver : :: std :: os :: raw :: c_int , # [ doc = "< Number of asynchronous engines" ] pub asyncEngineCount : :: std :: os :: raw :: c_int , # [ doc = "< Device shares a unified address space with the host" ] pub unifiedAddressing : :: std :: os :: raw :: c_int , # [ doc = "< Peak memory clock frequency in kilohertz" ] pub memoryClockRate : :: std :: os :: raw :: c_int , # [ doc = "< Global memory bus width in bits" ] pub memoryBusWidth : :: std :: os :: raw :: c_int , # [ doc = "< Size of L2 cache in bytes" ] pub l2CacheSize : :: std :: os :: raw :: c_int , # [ doc = "< Maximum resident threads per multiprocessor" ] pub maxThreadsPerMultiProcessor : :: std :: os :: raw :: c_int , # [ doc = "< Device supports stream priorities" ] pub streamPrioritiesSupported : :: std :: os :: raw :: c_int , # [ doc = "< Device supports caching globals in L1" ] pub globalL1CacheSupported : :: std :: os :: raw :: c_int , # [ doc = "< Device supports caching locals in L1" ] pub localL1CacheSupported : :: std :: os :: raw :: c_int , # [ doc = "< Shared memory available per multiprocessor in bytes" ] pub sharedMemPerMultiprocessor : usize , # [ doc = "< 32-bit registers available per multiprocessor" ] pub regsPerMultiprocessor : :: std :: os :: raw :: c_int , # [ doc = "< Device supports allocating managed memory on this system" ] pub managedMemory : :: std :: os :: raw :: c_int , # [ doc = "< Device is on a multi-GPU board" ] pub isMultiGpuBoard : :: std :: os :: raw :: c_int , # [ doc = "< Unique identifier for a group of devices on the same multi-GPU board" ] pub multiGpuBoardGroupID : :: std :: os :: raw :: c_int , # [ doc = "< Link between the device and the host supports native atomic operations" ] pub hostNativeAtomicSupported : :: std :: os :: raw :: c_int , # [ doc = "< Ratio of single precision performance (in floating-point operations per second) to double precision performance" ] pub singleToDoublePrecisionPerfRatio : :: std :: os :: raw :: c_int , # [ doc = "< Device supports coherently accessing pageable memory without calling cudaHostRegister on it" ] pub pageableMemoryAccess : :: std :: os :: raw :: c_int , # [ doc = "< Device can coherently access managed memory concurrently with the CPU" ] pub concurrentManagedAccess : :: std :: os :: raw :: c_int , # [ doc = "< Device supports Compute Preemption" ] pub computePreemptionSupported : :: std :: os :: raw :: c_int , # [ doc = "< Device can access host registered memory at the same virtual address as the CPU" ] pub canUseHostPointerForRegisteredMem : :: std :: os :: raw :: c_int , # [ doc = "< Device supports launching cooperative kernels via ::cudaLaunchCooperativeKernel" ] pub cooperativeLaunch : :: std :: os :: raw :: c_int , # [ doc = "< Device can participate in cooperative kernels launched via ::cudaLaunchCooperativeKernelMultiDevice" ] pub cooperativeMultiDeviceLaunch : :: std :: os :: raw :: c_int , # [ doc = "< Per device maximum shared memory per block usable by special opt in" ] pub sharedMemPerBlockOptin : usize , } # [ test ] fn bindgen_test_layout_cudaDeviceProp ( ) { assert_eq ! ( :: std :: mem :: size_of :: < cudaDeviceProp > ( ) , 672usize , concat ! ( "Size of: " , stringify ! ( cudaDeviceProp ) ) ) ; assert_eq ! ( :: std :: mem :: align_of :: < cudaDeviceProp > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( cudaDeviceProp ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( name ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . totalGlobalMem as * const _ as usize } , 256usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( totalGlobalMem ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . sharedMemPerBlock as * const _ as usize } , 264usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( sharedMemPerBlock ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . regsPerBlock as * const _ as usize } , 272usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( regsPerBlock ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . warpSize as * const _ as usize } , 276usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( warpSize ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . memPitch as * const _ as usize } , 280usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( memPitch ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxThreadsPerBlock as * const _ as usize } , 288usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxThreadsPerBlock ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxThreadsDim as * const _ as usize } , 292usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxThreadsDim ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxGridSize as * const _ as usize } , 304usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxGridSize ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . clockRate as * const _ as usize } , 316usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( clockRate ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . totalConstMem as * const _ as usize } , 320usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( totalConstMem ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . major as * const _ as usize } , 328usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( major ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . minor as * const _ as usize } , 332usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( minor ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . textureAlignment as * const _ as usize } , 336usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( textureAlignment ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . texturePitchAlignment as * const _ as usize } , 344usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( texturePitchAlignment ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . deviceOverlap as * const _ as usize } , 352usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( deviceOverlap ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . multiProcessorCount as * const _ as usize } , 356usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( multiProcessorCount ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . kernelExecTimeoutEnabled as * const _ as usize } , 360usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( kernelExecTimeoutEnabled ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . integrated as * const _ as usize } , 364usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( integrated ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . canMapHostMemory as * const _ as usize } , 368usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( canMapHostMemory ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . computeMode as * const _ as usize } , 372usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( computeMode ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture1D as * const _ as usize } , 376usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture1D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture1DMipmap as * const _ as usize } , 380usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture1DMipmap ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture1DLinear as * const _ as usize } , 384usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture1DLinear ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture2D as * const _ as usize } , 388usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture2D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture2DMipmap as * const _ as usize } , 396usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture2DMipmap ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture2DLinear as * const _ as usize } , 404usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture2DLinear ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture2DGather as * const _ as usize } , 416usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture2DGather ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture3D as * const _ as usize } , 424usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture3D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture3DAlt as * const _ as usize } , 436usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture3DAlt ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTextureCubemap as * const _ as usize } , 448usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTextureCubemap ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture1DLayered as * const _ as usize } , 452usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture1DLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTexture2DLayered as * const _ as usize } , 460usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTexture2DLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxTextureCubemapLayered as * const _ as usize } , 472usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxTextureCubemapLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurface1D as * const _ as usize } , 480usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurface1D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurface2D as * const _ as usize } , 484usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurface2D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurface3D as * const _ as usize } , 492usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurface3D ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurface1DLayered as * const _ as usize } , 504usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurface1DLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurface2DLayered as * const _ as usize } , 512usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurface2DLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurfaceCubemap as * const _ as usize } , 524usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurfaceCubemap ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxSurfaceCubemapLayered as * const _ as usize } , 528usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxSurfaceCubemapLayered ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . surfaceAlignment as * const _ as usize } , 536usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( surfaceAlignment ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . concurrentKernels as * const _ as usize } , 544usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( concurrentKernels ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . ECCEnabled as * const _ as usize } , 548usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( ECCEnabled ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . pciBusID as * const _ as usize } , 552usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( pciBusID ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . pciDeviceID as * const _ as usize } , 556usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( pciDeviceID ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . pciDomainID as * const _ as usize } , 560usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( pciDomainID ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . tccDriver as * const _ as usize } , 564usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( tccDriver ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . asyncEngineCount as * const _ as usize } , 568usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( asyncEngineCount ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . unifiedAddressing as * const _ as usize } , 572usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( unifiedAddressing ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . memoryClockRate as * const _ as usize } , 576usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( memoryClockRate ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . memoryBusWidth as * const _ as usize } , 580usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( memoryBusWidth ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . l2CacheSize as * const _ as usize } , 584usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( l2CacheSize ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . maxThreadsPerMultiProcessor as * const _ as usize } , 588usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( maxThreadsPerMultiProcessor ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . streamPrioritiesSupported as * const _ as usize } , 592usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( streamPrioritiesSupported ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . globalL1CacheSupported as * const _ as usize } , 596usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( globalL1CacheSupported ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . localL1CacheSupported as * const _ as usize } , 600usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( localL1CacheSupported ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . sharedMemPerMultiprocessor as * const _ as usize } , 608usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( sharedMemPerMultiprocessor ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . regsPerMultiprocessor as * const _ as usize } , 616usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( regsPerMultiprocessor ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . managedMemory as * const _ as usize } , 620usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( managedMemory ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . isMultiGpuBoard as * const _ as usize } , 624usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( isMultiGpuBoard ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . multiGpuBoardGroupID as * const _ as usize } , 628usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( multiGpuBoardGroupID ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . hostNativeAtomicSupported as * const _ as usize } , 632usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( hostNativeAtomicSupported ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . singleToDoublePrecisionPerfRatio as * const _ as usize } , 636usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( singleToDoublePrecisionPerfRatio ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . pageableMemoryAccess as * const _ as usize } , 640usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( pageableMemoryAccess ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . concurrentManagedAccess as * const _ as usize } , 644usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( concurrentManagedAccess ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . computePreemptionSupported as * const _ as usize } , 648usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( computePreemptionSupported ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . canUseHostPointerForRegisteredMem as * const _ as usize } , 652usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( canUseHostPointerForRegisteredMem ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . cooperativeLaunch as * const _ as usize } , 656usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( cooperativeLaunch ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . cooperativeMultiDeviceLaunch as * const _ as usize } , 660usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( cooperativeMultiDeviceLaunch ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < cudaDeviceProp > ( ) ) ) . sharedMemPerBlockOptin as * const _ as usize } , 664usize , concat ! ( "Offset of field: " , stringify ! ( cudaDeviceProp ) , "::" , stringify ! ( sharedMemPerBlockOptin ) ) ) ; } # [ doc = " Type of stream callback functions." ] # [ doc = " \\param stream The stream as passed to ::cudaStreamAddCallback, may be NULL." ] # [ doc = " \\param status ::cudaSuccess or any persistent error on the stream." ] # [ doc = " \\param userData User parameter provided at registration." ] pub type cudaStreamCallback_t = :: std :: option :: Option < unsafe extern "C" fn ( stream : cudaStream_t , status : cudaError_t , userData : * mut :: std :: os :: raw :: c_void ) > ;