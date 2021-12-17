# use-after-free in WebCore::ContainerNode::firstChild()

report id: Bug 228258

When the [html file](https://github.com/ChijinZ/security_advisories/blob/master/webkitgtk-2.32.3/seeds/uaf-firstchild.html) is input to webkitgtk, Asan reports the heap-use-after-free message. 

```
=================================================================
==67206==ERROR: AddressSanitizer: heap-use-after-free on address 0x61200007b6a0 at pc 0x7f85d2632da2 bp 0x7ffe2e4b7580 sp 0x7ffe2e4b7578
READ of size 8 at 0x61200007b6a0 thread T0
    #0 0x7f85d2632da1 in WebCore::ContainerNode::firstChild() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/WebCore/ContainerNode.h:43:39
    #1 0x7f85d5eb0e24 in WebCore::SVGElement* WebCore::Traversal<WebCore::SVGElement>::firstWithinTemplate<WebCore::ContainerNode const>(WebCore::ContainerNode const&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ElementTraversal.h:137:26
    #2 0x7f85d5eb0dd4 in WebCore::Traversal<WebCore::SVGElement>::firstWithin(WebCore::ContainerNode const&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ElementTraversal.h:238:96
    #3 0x7f85d5e799dd in WebCore::ElementDescendantRange<WebCore::SVGElement>::begin() const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/TypedElementDescendantIterator.h:146:59
    #4 0x7f85da75b43a in WebCore::disassociateAndRemoveClones(WTF::Vector<WebCore::Element*, 0ul, WTF::CrashOnOverflow, 16ul, WTF::FastMalloc> const&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:315:31
    #5 0x7f85da758545 in WebCore::removeSymbolElementsFromSubtree(WebCore::SVGElement&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:355:5
    #6 0x7f85da756129 in WebCore::SVGUseElement::cloneTarget(WebCore::ContainerNode&, WebCore::SVGElement&) const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:434:5
    #7 0x7f85da7565d9 in WebCore::SVGUseElement::expandUseElementsInShadowTree() const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:475:27
    #8 0x7f85da755708 in WebCore::SVGUseElement::updateShadowTree() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:240:9
    #9 0x7f85d6bbe62b in WebCore::Document::resolveStyle(WebCore::Document::ResolveStyleType) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:2010:22
    #10 0x7f85d6bbf402 in WebCore::Document::updateStyleIfNeeded() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:2156:5
    #11 0x7f85d6be86b5 in WebCore::Document::finishedParsing() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:5993:9
    #12 0x7f85d79bde19 in WebCore::HTMLConstructionSite::finishedParsing() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLConstructionSite.cpp:419:16
    #13 0x7f85d7a33984 in WebCore::HTMLTreeBuilder::finished() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLTreeBuilder.cpp:2843:12
    #14 0x7f85d79c77bc in WebCore::HTMLDocumentParser::end() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLDocumentParser.cpp:448:20
    #15 0x7f85d79c508a in WebCore::HTMLDocumentParser::attemptToRunDeferredScriptsAndEnd() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLDocumentParser.cpp:457:5
    #16 0x7f85d79c4da3 in WebCore::HTMLDocumentParser::prepareToStopParsing() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLDocumentParser.cpp:152:5
    #17 0x7f85d79c5de0 in WebCore::HTMLDocumentParser::endIfDelayed() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLDocumentParser.cpp:482:5
    #18 0x7f85d79c5c94 in WebCore::HTMLDocumentParser::resumeParsingAfterYield() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLDocumentParser.cpp:214:5
    #19 0x7f85d79f4c4f in WebCore::HTMLParserScheduler::continueNextChunkTimerFired() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLParserScheduler.cpp:104:14
    #20 0x7f85d7a05cbc in void std::__invoke_impl<void, void (WebCore::HTMLParserScheduler::*&)(), WebCore::HTMLParserScheduler*&>(std::__invoke_memfun_deref, void (WebCore::HTMLParserScheduler::*&)(), WebCore::HTMLParserScheduler*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:73:14
    #21 0x7f85d7a05b81 in std::__invoke_result<void (WebCore::HTMLParserScheduler::*&)(), WebCore::HTMLParserScheduler*&>::type std::__invoke<void (WebCore::HTMLParserScheduler::*&)(), WebCore::HTMLParserScheduler*&>(void (WebCore::HTMLParserScheduler::*&)(), WebCore::HTMLParserScheduler*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:95:14
    #22 0x7f85d7a05af4 in void std::_Bind<void (WebCore::HTMLParserScheduler::* (WebCore::HTMLParserScheduler*))()>::__call<void, 0ul>(std::tuple<>&&, std::_Index_tuple<0ul>) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:400:11
    #23 0x7f85d7a0599d in void std::_Bind<void (WebCore::HTMLParserScheduler::* (WebCore::HTMLParserScheduler*))()>::operator()<void>() /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:482:17
    #24 0x7f85d7a05808 in WTF::Detail::CallableWrapper<std::_Bind<void (WebCore::HTMLParserScheduler::* (WebCore::HTMLParserScheduler*))()>, void>::call() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:52:39
    #25 0x7f85d0282fd2 in WTF::Function<void ()>::operator()() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:83:35
    #26 0x7f85d0f56658 in WebCore::Timer::fired() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/WebCore/Timer.h:136:9
    #27 0x7f85d89e22c7 in WebCore::ThreadTimers::sharedTimerFiredInternal() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/platform/ThreadTimers.cpp:127:23
    #28 0x7f85d89e6640 in WebCore::ThreadTimers::setSharedTimer(WebCore::SharedTimer*)::$_0::operator()() const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/platform/ThreadTimers.cpp:67:80
    #29 0x7f85d89e6618 in WTF::Detail::CallableWrapper<WebCore::ThreadTimers::setSharedTimer(WebCore::SharedTimer*)::$_0, void>::call() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:52:39
    #30 0x7f85d0282fd2 in WTF::Function<void ()>::operator()() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:83:35
    #31 0x7f85d8948fe5 in WebCore::MainThreadSharedTimer::fired() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/platform/MainThreadSharedTimer.cpp:83:5
    #32 0x7f85d895804c in void std::__invoke_impl<void, void (WebCore::MainThreadSharedTimer::*&)(), WebCore::MainThreadSharedTimer*&>(std::__invoke_memfun_deref, void (WebCore::MainThreadSharedTimer::*&)(), WebCore::MainThreadSharedTimer*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:73:14
    #33 0x7f85d8957f11 in std::__invoke_result<void (WebCore::MainThreadSharedTimer::*&)(), WebCore::MainThreadSharedTimer*&>::type std::__invoke<void (WebCore::MainThreadSharedTimer::*&)(), WebCore::MainThreadSharedTimer*&>(void (WebCore::MainThreadSharedTimer::*&)(), WebCore::MainThreadSharedTimer*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:95:14
    #34 0x7f85d8957e84 in void std::_Bind<void (WebCore::MainThreadSharedTimer::* (WebCore::MainThreadSharedTimer*))()>::__call<void, 0ul>(std::tuple<>&&, std::_Index_tuple<0ul>) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:400:11
    #35 0x7f85d8957d2d in void std::_Bind<void (WebCore::MainThreadSharedTimer::* (WebCore::MainThreadSharedTimer*))()>::operator()<void>() /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:482:17
    #36 0x7f85d8957af8 in WTF::Detail::CallableWrapper<std::_Bind<void (WebCore::MainThreadSharedTimer::* (WebCore::MainThreadSharedTimer*))()>, void>::call() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:52:39
    #37 0x7f85d0282fd2 in WTF::Function<void ()>::operator()() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Function.h:83:35
    #38 0x7f85d8957158 in WTF::RunLoop::Timer<WebCore::MainThreadSharedTimer>::fired() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/RunLoop.h:186:33
    #39 0x7f85ca152f74 in WTF::RunLoop::TimerBase::TimerBase(WTF::RunLoop&)::$_3::operator()(void*) const /path/to/webkitgtk-2.32.0/asan_build/../Source/WTF/wtf/glib/RunLoopGLib.cpp:177:16
    #40 0x7f85ca152e94 in WTF::RunLoop::TimerBase::TimerBase(WTF::RunLoop&)::$_3::__invoke(void*) /path/to/webkitgtk-2.32.0/asan_build/../Source/WTF/wtf/glib/RunLoopGLib.cpp:169:43
    #41 0x7f85ca152ddd in WTF::RunLoop::$_0::operator()(_GSource*, int (*)(void*), void*) const /path/to/webkitgtk-2.32.0/asan_build/../Source/WTF/wtf/glib/RunLoopGLib.cpp:53:28
    #42 0x7f85ca150e64 in WTF::RunLoop::$_0::__invoke(_GSource*, int (*)(void*), void*) /path/to/webkitgtk-2.32.0/asan_build/../Source/WTF/wtf/glib/RunLoopGLib.cpp:45:5
    #43 0x7f85bff9204d in g_main_context_dispatch (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x5204d)
    #44 0x7f85bff923ff  (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x523ff)
    #45 0x7f85bff926f2 in g_main_loop_run (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x526f2)
    #46 0x7f85ca15177b in WTF::RunLoop::run() /path/to/webkitgtk-2.32.0/asan_build/../Source/WTF/wtf/glib/RunLoopGLib.cpp:108:9
    #47 0x7f85d2b86482 in WebKit::AuxiliaryProcessMainBase<WebKit::WebProcess, true>::run(int, char**) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebKit/Shared/AuxiliaryProcessMain.h:70:9
    #48 0x7f85d2b804bd in int WebKit::AuxiliaryProcessMain<WebKit::WebProcessMainGtk>(int, char**) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebKit/Shared/AuxiliaryProcessMain.h:96:27
    #49 0x7f85d2b7f0e6 in WebKit::WebProcessMain(int, char**) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebKit/WebProcess/gtk/WebProcessMainGtk.cpp:78:12
    #50 0x4fcaf1 in main /path/to/webkitgtk-2.32.0/asan_build/../Source/WebKit/WebProcess/EntryPoint/unix/WebProcessMain.cpp:31:12
    #51 0x7f85bf92c0b2 in __libc_start_main /build/glibc-eX1tMB/glibc-2.31/csu/../csu/libc-start.c:308:16
    #52 0x41d34d in _start (/path/to/webkitgtk-2.32.0/asan_build/INSTALL/libexec/webkit2gtk-4.0/WebKitWebProcess+0x41d34d)

0x61200007b6a0 is located 96 bytes inside of 304-byte region [0x61200007b640,0x61200007b770)
freed by thread T0 here:
    #0 0x4c3107 in free /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:127:3
    #1 0x7f85ca186f88 in bmalloc::DebugHeap::free(void*) /path/to/webkitgtk-2.32.0/asan_build/../Source/bmalloc/bmalloc/DebugHeap.cpp:120:5
    #2 0x7f85ca1838ca in bmalloc::Cache::deallocateSlowCaseNullCache(bmalloc::HeapKind, void*) /path/to/webkitgtk-2.32.0/asan_build/../Source/bmalloc/bmalloc/Cache.cpp:85:20
    #3 0x7f85d526778e in bmalloc::Cache::deallocate(bmalloc::HeapKind, void*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/Cache.h:105:16
    #4 0x7f85d526774a in bmalloc::api::free(void*, bmalloc::HeapKind) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/bmalloc.h:86:5
    #5 0x7f85da6b3ae0 in void bmalloc::IsoTLS::deallocateSlow<bmalloc::IsoConfig<304u>, WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, void*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:145:20
    #6 0x7f85da6b3952 in void bmalloc::IsoTLS::deallocateImpl<bmalloc::IsoConfig<304u>, WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, void*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:122:9
    #7 0x7f85da6b38ac in void bmalloc::IsoTLS::deallocate<WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, void*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:50:5
    #8 0x7f85da66c00c in bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>::deallocate(void*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoHeapInlines.h:73:5
    #9 0x7f85da6624fc in WebCore::SVGSymbolElement::operator delete(void*) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.cpp:32:1
    #10 0x7f85da66e151 in WebCore::SVGSymbolElement::~SVGSymbolElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.h:29:7
    #11 0x7f85d6ed24ab in WebCore::Node::removedLastRef() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Node.cpp:2550:5
    #12 0x7f85d160766b in WebCore::Node::deref() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/WebCore/Node.h:815:34
    #13 0x7f85d16072fd in WTF::DefaultRefDerefTraits<WebCore::Node>::derefIfNotNull(WebCore::Node*) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/RefPtr.h:42:18
    #14 0x7f85d160726e in WTF::RefPtr<WebCore::Node, WTF::RawPtrTraits<WebCore::Node>, WTF::DefaultRefDerefTraits<WebCore::Node> >::~RefPtr() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/RefPtr.h:73:31
    #15 0x7f85d5d7938e in WTF::RefPtr<WebCore::Node, WTF::RawPtrTraits<WebCore::Node>, WTF::DefaultRefDerefTraits<WebCore::Node> >::operator=(WTF::RefPtr<WebCore::Node, WTF::RawPtrTraits<WebCore::Node>, WTF::DefaultRefDerefTraits<WebCore::Node> > const&) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/RefPtr.h:137:1
    #16 0x7f85d6b54a40 in WebCore::addChildNodesToDeletionQueue(WebCore::Node*&, WebCore::Node*&, WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNodeAlgorithms.cpp:190:65
    #17 0x7f85d6b54c21 in WebCore::removeDetachedChildrenInContainer(WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNodeAlgorithms.cpp:229:5
    #18 0x7f85d6af1fd6 in WebCore::ContainerNode::removeDetachedChildren() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:281:5
    #19 0x7f85d6af28a9 in WebCore::ContainerNode::~ContainerNode() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:315:5
    #20 0x7f85d6d5ba18 in WebCore::Element::~Element() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Element.cpp:233:1
    #21 0x7f85d704451f in WebCore::StyledElement::~StyledElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/StyledElement.cpp:72:1
    #22 0x7f85da160437 in WebCore::SVGElement::~SVGElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGElement.cpp:180:1
    #23 0x7f85da420706 in WebCore::SVGGraphicsElement::~SVGGraphicsElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGGraphicsElement.cpp:51:41
    #24 0x7f85da66e128 in WebCore::SVGSymbolElement::~SVGSymbolElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.h:29:7
    #25 0x7f85da66e148 in WebCore::SVGSymbolElement::~SVGSymbolElement() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.h:29:7
    #26 0x7f85d6ed24ab in WebCore::Node::removedLastRef() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Node.cpp:2550:5
    #27 0x7f85d160766b in WebCore::Node::deref() const /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/WebCore/Node.h:815:34
    #28 0x7f85d160795c in WTF::Ref<WebCore::Node, WTF::RawPtrTraits<WebCore::Node> >::~Ref() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/wtf/Ref.h:61:39
    #29 0x7f85d6b04c31 in WebCore::ContainerNode::removeNodeWithScriptAssertion(WebCore::Node&, WebCore::ContainerNode::ChildChange::Source) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:203:1

previously allocated by thread T0 here:
    #0 0x4c33ff in malloc /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:145:3
    #1 0x7f85ca186d1b in bmalloc::DebugHeap::malloc(unsigned long, bmalloc::FailureAction) /path/to/webkitgtk-2.32.0/asan_build/../Source/bmalloc/bmalloc/DebugHeap.cpp:98:20
    #2 0x7f85ca1832af in bmalloc::Cache::tryAllocateSlowCaseNullCache(bmalloc::HeapKind, unsigned long) /path/to/webkitgtk-2.32.0/asan_build/../Source/bmalloc/bmalloc/Cache.cpp:57:27
    #3 0x7f85d525144e in bmalloc::Cache::tryAllocate(bmalloc::HeapKind, unsigned long) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/Cache.h:73:16
    #4 0x7f85d525103a in bmalloc::api::tryMalloc(unsigned long, bmalloc::HeapKind) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/bmalloc.h:43:12
    #5 0x7f85da6b30c0 in void* bmalloc::IsoTLS::allocateSlow<bmalloc::IsoConfig<304u>, WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, bool) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:98:20
    #6 0x7f85da6b2f7a in void* bmalloc::IsoTLS::allocateImpl<bmalloc::IsoConfig<304u>, WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, bool) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:76:16
    #7 0x7f85da6b2ed4 in void* bmalloc::IsoTLS::allocate<WebCore::SVGSymbolElement>(bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>&, bool) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoTLSInlines.h:42:12
    #8 0x7f85da66bfe0 in bmalloc::api::IsoHeap<WebCore::SVGSymbolElement>::allocate() /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/bmalloc/IsoHeapInlines.h:60:12
    #9 0x7f85da6624d2 in WebCore::SVGSymbolElement::operator new(unsigned long) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.cpp:32:1
    #10 0x7f85da662538 in WebCore::SVGSymbolElement::create(WebCore::QualifiedName const&, WebCore::Document&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGSymbolElement.cpp:43:22
    #11 0x7f85d50edc3e in WebCore::symbolConstructor(WebCore::QualifiedName const&, WebCore::Document&, bool) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/WebCore/SVGElementFactory.cpp:484:12
    #12 0x7f85d50a1281 in WebCore::SVGElementFactory::createElement(WebCore::QualifiedName const&, WebCore::Document&, bool) /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/WebCore/SVGElementFactory.cpp:679:16
    #13 0x7f85d6bb3708 in WebCore::Document::createElement(WebCore::QualifiedName const&, bool) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:1235:19
    #14 0x7f85d6d5f7f1 in WebCore::Element::cloneElementWithoutAttributesAndChildren(WebCore::Document&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Element.cpp:501:27
    #15 0x7f85d6d5f5dd in WebCore::Element::cloneElementWithoutChildren(WebCore::Document&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Element.cpp:489:26
    #16 0x7f85d6d5f483 in WebCore::Element::cloneNodeInternal(WebCore::Document&, WebCore::Node::CloningOperation) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Element.cpp:473:16
    #17 0x7f85d6afae41 in WebCore::ContainerNode::cloneChildNodes(WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:838:35
    #18 0x7f85d6afaeed in WebCore::ContainerNode::cloneChildNodes(WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:840:45
    #19 0x7f85d6afaeed in WebCore::ContainerNode::cloneChildNodes(WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:840:45
    #20 0x7f85d6afaeed in WebCore::ContainerNode::cloneChildNodes(WebCore::ContainerNode&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/ContainerNode.cpp:840:45
    #21 0x7f85d6d5f6f3 in WebCore::Element::cloneElementWithChildren(WebCore::Document&) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Element.cpp:483:5
    #22 0x7f85da7560c9 in WebCore::SVGUseElement::cloneTarget(WebCore::ContainerNode&, WebCore::SVGElement&) const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:431:67
    #23 0x7f85da7565d9 in WebCore::SVGUseElement::expandUseElementsInShadowTree() const /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:475:27
    #24 0x7f85da755708 in WebCore::SVGUseElement::updateShadowTree() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/svg/SVGUseElement.cpp:240:9
    #25 0x7f85d6bbe62b in WebCore::Document::resolveStyle(WebCore::Document::ResolveStyleType) /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:2010:22
    #26 0x7f85d6bbf402 in WebCore::Document::updateStyleIfNeeded() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:2156:5
    #27 0x7f85d6be86b5 in WebCore::Document::finishedParsing() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/dom/Document.cpp:5993:9
    #28 0x7f85d79bde19 in WebCore::HTMLConstructionSite::finishedParsing() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLConstructionSite.cpp:419:16
    #29 0x7f85d7a33984 in WebCore::HTMLTreeBuilder::finished() /path/to/webkitgtk-2.32.0/asan_build/../Source/WebCore/html/parser/HTMLTreeBuilder.cpp:2843:12

SUMMARY: AddressSanitizer: heap-use-after-free /path/to/webkitgtk-2.32.0/asan_build/DerivedSources/ForwardingHeaders/WebCore/ContainerNode.h:43:39 in WebCore::ContainerNode::firstChild() const
Shadow bytes around the buggy address:
  0x0c2480007680: fd fd fd fd fd fd fd fd fd fd fd fa fa fa fa fa
  0x0c2480007690: fa fa fa fa fa fa fa fa fd fd fd fd fd fd fd fd
  0x0c24800076a0: fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd
  0x0c24800076b0: fd fd fd fd fd fd fd fd fd fd fd fa fa fa fa fa
  0x0c24800076c0: fa fa fa fa fa fa fa fa fd fd fd fd fd fd fd fd
=>0x0c24800076d0: fd fd fd fd[fd]fd fd fd fd fd fd fd fd fd fd fd
  0x0c24800076e0: fd fd fd fd fd fd fd fd fd fd fd fd fd fd fa fa
  0x0c24800076f0: fa fa fa fa fa fa fa fa fd fd fd fd fd fd fd fd
  0x0c2480007700: fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd
  0x0c2480007710: fd fd fd fd fd fd fd fd fd fa fa fa fa fa fa fa
  0x0c2480007720: fa fa fa fa fa fa fa fa fd fd fd fd fd fd fd fd
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
  Shadow gap:              cc
==67206==ABORTING
```

# heap-use-after-free in WebCore::Frame::page()

report id: Bug 228883

When the [html file](https://github.com/ChijinZ/security_advisories/blob/master/webkitgtk-2.32.3/seeds/uaf-page.html) is input to webkitgtk, Asan reports the heap-use-after-free message. 

```
==98422==ERROR: AddressSanitizer: heap-use-after-free on address 0x6130001082d8 at pc 0x7f7f660c2b7e bp 0x7ffc3cd6a220 sp 0x7ffc3cd6a218
READ of size 8 at 0x6130001082d8 thread T0
    #0 0x7f7f660c2b7d in WTF::RefPtr<WTF::WeakPtrImpl<WTF::EmptyCounter>, WTF::RawPtrTraits<WTF::WeakPtrImpl<WTF::EmptyCounter> >, WTF::DefaultRefDerefTraits<WTF::WeakPtrImpl<WTF::EmptyCounter> > >::operator!() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RefPtr.h:87:38
    #1 0x7f7f6f1f43cc in WTF::WeakPtr<WebCore::Page, WTF::EmptyCounter>::get() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/WeakPtr.h:105:9
    #2 0x7f7f6fb10398 in WebCore::Frame::page() const /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Frame.cpp:225:19
    #3 0x7f7f6d542a94 in WebCore::InspectorInstrumentation::instrumentingAgents(WebCore::Frame const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/inspector/InspectorInstrumentation.h:1829:38
    #4 0x7f7f6f262c6f in WebCore::InspectorInstrumentation::instrumentingAgents(WebCore::Frame const*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/inspector/InspectorInstrumentation.h:1824:20
    #5 0x7f7f6fa39845 in WebCore::InspectorInstrumentation::didDispatchEventOnWindow(WebCore::Frame*, WebCore::Event const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/inspector/InspectorInstrumentation.h:951:24
    #6 0x7f7f6fa0d60b in WebCore::DOMWindow::dispatchEvent(WebCore::Event&, WebCore::EventTarget*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/DOMWindow.cpp:2320:9
    #7 0x7f7f6e0513d8 in WebCore::Document::dispatchWindowEvent(WebCore::Event&, WebCore::EventTarget*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/Document.cpp:5012:18
    #8 0x7f7f6e050fdf in WebCore::Document::runResizeSteps() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/Document.cpp:4259:9
    #9 0x7f7f6fc6b868 in WebCore::Page::updateRendering()::$_23::operator()(WebCore::Document&) const /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Page.cpp:1574:18
    #10 0x7f7f6fc6b820 in WTF::Detail::CallableWrapper<WebCore::Page::updateRendering()::$_23, void, WebCore::Document&>::call(WebCore::Document&) /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:53:39
    #11 0x7f7f6fc9a3be in WTF::Function<void (WebCore::Document&)>::operator()(WebCore::Document&) const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:82:35
    #12 0x7f7f6fc2c469 in WebCore::Page::forEachDocument(WTF::Function<void (WebCore::Document&)> const&) const /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Page.cpp:3376:9
    #13 0x7f7f6fc3d5fa in WebCore::Page::updateRendering()::$_22::operator()(WebCore::RenderingUpdateStep, WTF::Function<void (WebCore::Document&)> const&) const /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Page.cpp:1568:9
    #14 0x7f7f6fc3cd26 in WebCore::Page::updateRendering() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Page.cpp:1573:5
    #15 0x7f7f68d15250 in WebKit::WebPage::updateRendering() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebPage/WebPage.cpp:4263:13
    #16 0x7f7f68e21f95 in WebKit::DrawingAreaCoordinatedGraphics::display(WebKit::UpdateInfo&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebPage/CoordinatedGraphics/DrawingAreaCoordinatedGraphics.cpp:826:15
    #17 0x7f7f68e1b011 in WebKit::DrawingAreaCoordinatedGraphics::display() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebPage/CoordinatedGraphics/DrawingAreaCoordinatedGraphics.cpp:784:5
    #18 0x7f7f68e18944 in WebKit::DrawingAreaCoordinatedGraphics::displayTimerFired() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebPage/CoordinatedGraphics/DrawingAreaCoordinatedGraphics.cpp:763:5
    #19 0x7f7f68e42f6c in void std::__invoke_impl<void, void (WebKit::DrawingAreaCoordinatedGraphics::*&)(), WebKit::DrawingAreaCoordinatedGraphics*&>(std::__invoke_memfun_deref, void (WebKit::DrawingAreaCoordinatedGraphics::*&)(), WebKit::DrawingAreaCoordinatedGraphics*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:73:14
    #20 0x7f7f68e42e31 in std::__invoke_result<void (WebKit::DrawingAreaCoordinatedGraphics::*&)(), WebKit::DrawingAreaCoordinatedGraphics*&>::type std::__invoke<void (WebKit::DrawingAreaCoordinatedGraphics::*&)(), WebKit::DrawingAreaCoordinatedGraphics*&>(void (WebKit::DrawingAreaCoordinatedGraphics::*&)(), WebKit::DrawingAreaCoordinatedGraphics*&) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/invoke.h:95:14
    #21 0x7f7f68e42da4 in void std::_Bind<void (WebKit::DrawingAreaCoordinatedGraphics::* (WebKit::DrawingAreaCoordinatedGraphics*))()>::__call<void, 0ul>(std::tuple<>&&, std::_Index_tuple<0ul>) /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:400:11
    #22 0x7f7f68e42c4d in void std::_Bind<void (WebKit::DrawingAreaCoordinatedGraphics::* (WebKit::DrawingAreaCoordinatedGraphics*))()>::operator()<void>() /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/functional:482:17
    #23 0x7f7f68e42a08 in WTF::Detail::CallableWrapper<std::_Bind<void (WebKit::DrawingAreaCoordinatedGraphics::* (WebKit::DrawingAreaCoordinatedGraphics*))()>, void>::call() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:53:39
    #24 0x7f7f661b3512 in WTF::Function<void ()>::operator()() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:82:35
    #25 0x7f7f68e37dd8 in WTF::RunLoop::Timer<WebKit::DrawingAreaCoordinatedGraphics>::fired() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RunLoop.h:188:33
    #26 0x7f7f5f3bc674 in WTF::RunLoop::TimerBase::TimerBase(WTF::RunLoop&)::$_3::operator()(void*) const /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/glib/RunLoopGLib.cpp:177:16
    #27 0x7f7f5f3bc594 in WTF::RunLoop::TimerBase::TimerBase(WTF::RunLoop&)::$_3::__invoke(void*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/glib/RunLoopGLib.cpp:169:43
    #28 0x7f7f5f3bc4dd in WTF::RunLoop::$_0::operator()(_GSource*, int (*)(void*), void*) const /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/glib/RunLoopGLib.cpp:53:28
    #29 0x7f7f5f3ba564 in WTF::RunLoop::$_0::__invoke(_GSource*, int (*)(void*), void*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/glib/RunLoopGLib.cpp:45:5
    #30 0x7f7f56be404d in g_main_context_dispatch (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x5204d)
    #31 0x7f7f56be43ff  (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x523ff)
    #32 0x7f7f56be46f2 in g_main_loop_run (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0+0x526f2)
    #33 0x7f7f5f3bae7b in WTF::RunLoop::run() /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/glib/RunLoopGLib.cpp:108:9
    #34 0x7f7f68e91412 in WebKit::AuxiliaryProcessMainBase<WebKit::WebProcess, true>::run(int, char**) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/Shared/AuxiliaryProcessMain.h:70:9
    #35 0x7f7f68e7e91e in int WebKit::AuxiliaryProcessMain<WebKit::WebProcessMainGtk>(int, char**) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/Shared/AuxiliaryProcessMain.h:96:27
    #36 0x7f7f68e7b966 in WebKit::WebProcessMain(int, char**) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/gtk/WebProcessMainGtk.cpp:78:12
    #37 0x4fcb61 in main /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/EntryPoint/unix/WebProcessMain.cpp:31:12
    #38 0x7f7f565780b2 in __libc_start_main /build/glibc-eX1tMB/glibc-2.31/csu/../csu/libc-start.c:308:16
    #39 0x41d3bd in _start (/path/to/WebKitBuild/GTK/Debug/INSTALL/bin/WebKitWebProcess+0x41d3bd)

0x6130001082d8 is located 88 bytes inside of 328-byte region [0x613000108280,0x6130001083c8)
freed by thread T0 here:
    #0 0x4c3177 in free /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:127:3
    #1 0x7f7f5f3e65e8 in bmalloc::DebugHeap::free(void*) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/DebugHeap.cpp:124:5
    #2 0x7f7f5f3e1a02 in bmalloc::Cache::deallocateSlowCaseNullCache(bmalloc::HeapKind, void*) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/Cache.cpp:85:20
    #3 0x7f7f5f1652fe in bmalloc::Cache::deallocate(bmalloc::HeapKind, void*) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/Cache.h:105:16
    #4 0x7f7f5f164046 in bmalloc::api::free(void*, bmalloc::HeapKind) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/bmalloc.h:150:5
    #5 0x7f7f5f164046 in WTF::fastFree(void*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/FastMalloc.cpp:558:5
    #6 0x7f7f660b9ca4 in WTF::ThreadSafeRefCountedBase::operator delete(void*) /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/ThreadSafeRefCounted.h:43:5
    #7 0x7f7f6fb1c471 in WebCore::Frame::~Frame() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Frame.cpp:198:1
    #8 0x7f7f6890573b in WTF::ThreadSafeRefCounted<WebCore::AbstractFrame, (WTF::DestructionThread)1>::deref() const::'lambda'()::operator()() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/ThreadSafeRefCounted.h:117:13
    #9 0x7f7f68905698 in WTF::Detail::CallableWrapper<WTF::ThreadSafeRefCounted<WebCore::AbstractFrame, (WTF::DestructionThread)1>::deref() const::'lambda'(), void>::call() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:53:39
    #10 0x7f7f5accd2f2 in WTF::Function<void ()>::operator()() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Function.h:82:35
    #11 0x7f7f5f1a1fb6 in WTF::ensureOnMainThread(WTF::Function<void ()>&&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/MainThread.cpp:94:9
    #12 0x7f7f6890519a in WTF::ThreadSafeRefCounted<WebCore::AbstractFrame, (WTF::DestructionThread)1>::deref() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/ThreadSafeRefCounted.h:123:13
    #13 0x7f7f689c999b in WTF::Ref<WebCore::Frame, WTF::RawPtrTraits<WebCore::Frame> >::~Ref() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/Ref.h:61:18
    #14 0x7f7f6fb358d4 in WebCore::FrameView::~FrameView() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/FrameView.cpp:244:1
    #15 0x7f7f6fb35a68 in WebCore::FrameView::~FrameView() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/FrameView.cpp:230:1
    #16 0x7f7f68ab7ec6 in std::default_delete<WebCore::Widget>::operator()(WebCore::Widget*) const /usr/lib/gcc/x86_64-linux-gnu/9/../../../../include/c++/9/bits/unique_ptr.h:81:2
    #17 0x7f7f68a9b26c in WTF::RefCounted<WebCore::Widget, std::default_delete<WebCore::Widget> >::deref() const /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RefCounted.h:190:13
    #18 0x7f7f6e9e02a1 in WTF::DefaultRefDerefTraits<WebCore::Widget>::derefIfNotNull(WebCore::Widget*) /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RefPtr.h:43:18
    #19 0x7f7f6e9de03e in WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >::~RefPtr() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RefPtr.h:75:31
    #20 0x7f7f714e6384 in WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*>::~KeyValuePair() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/KeyValuePair.h:33:8
    #21 0x7f7f714e630b in WTF::HashTable<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*>, WTF::KeyValuePairKeyExtractor<WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*> >, WTF::DefaultHash<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashMap<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*, WTF::DefaultHash<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WebCore::FrameView*>, WTF::HashTableTraits>::KeyValuePairTraits, WTF::HashTraits<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > > >::deallocateTable(WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*>*) /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/HashTable.h:1227:27
    #22 0x7f7f714e5f96 in WTF::HashTable<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*>, WTF::KeyValuePairKeyExtractor<WTF::KeyValuePair<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*> >, WTF::DefaultHash<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashMap<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*, WTF::DefaultHash<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WebCore::FrameView*>, WTF::HashTableTraits>::KeyValuePairTraits, WTF::HashTraits<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > > >::~HashTable() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/HashTable.h:415:17
    #23 0x7f7f714e1514 in WTF::HashMap<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> >, WebCore::FrameView*, WTF::DefaultHash<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WTF::RefPtr<WebCore::Widget, WTF::RawPtrTraits<WebCore::Widget>, WTF::DefaultRefDerefTraits<WebCore::Widget> > >, WTF::HashTraits<WebCore::FrameView*>, WTF::HashTableTraits>::~HashMap() /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/HashMap.h:35:7
    #24 0x7f7f714bb0f1 in WebCore::WidgetHierarchyUpdatesSuspensionScope::moveWidgets() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderWidget.cpp:73:5
    #25 0x7f7f6df82610 in WebCore::WidgetHierarchyUpdatesSuspensionScope::~WidgetHierarchyUpdatesSuspensionScope() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderWidget.h:41:13
    #26 0x7f7f6df7160c in WebCore::ContainerNode::removeNodeWithScriptAssertion(WebCore::Node&, WebCore::ContainerNode::ChildChange::Source) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:192:5
    #27 0x7f7f6df5e7fe in WebCore::ContainerNode::removeChild(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:614:10
    #28 0x7f7f6df5e406 in WebCore::ContainerNode::removeSelfOrChildNodesForInsertion(WebCore::Node&, WTF::Vector<WTF::Ref<WebCore::Node, WTF::RawPtrTraits<WebCore::Node> >, 11ul, WTF::CrashOnOverflow, 16ul, WTF::FastMalloc>&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:255:27
    #29 0x7f7f6df60813 in WebCore::ContainerNode::appendChildWithoutPreInsertionValidityCheck(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:740:25
    #30 0x7f7f6df66222 in WebCore::ContainerNode::appendChild(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:732:12

previously allocated by thread T0 here:
    #0 0x4c346f in malloc /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:145:3
    #1 0x7f7f5f3e637b in bmalloc::DebugHeap::malloc(unsigned long, bmalloc::FailureAction) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/DebugHeap.cpp:102:20
    #2 0x7f7f5f3e1684 in bmalloc::Cache::allocateSlowCaseNullCache(bmalloc::HeapKind, unsigned long) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/Cache.cpp:64:27
    #3 0x7f7f5f164a8e in bmalloc::Cache::allocate(bmalloc::HeapKind, unsigned long) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/Cache.h:81:16
    #4 0x7f7f5f1638e5 in bmalloc::api::malloc(unsigned long, bmalloc::HeapKind) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/bmalloc.h:78:12
    #5 0x7f7f5f1638e5 in WTF::fastMalloc(unsigned long) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/FastMalloc.cpp:525:20
    #6 0x7f7f66221b84 in WTF::ThreadSafeRefCountedBase::operator new(unsigned long) /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/ThreadSafeRefCounted.h:43:5
    #7 0x7f7f6fb1bbe7 in WebCore::Frame::create(WebCore::Page*, WebCore::HTMLFrameOwnerElement*, WTF::UniqueRef<WebCore::FrameLoaderClient>&&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/page/Frame.cpp:194:22
    #8 0x7f7f68dda1a8 in WebKit::WebFrame::createSubframe(WebKit::WebPage*, WTF::String const&, WebCore::HTMLFrameOwnerElement*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebPage/WebFrame.cpp:121:22
    #9 0x7f7f68bf5e3b in WebKit::WebFrameLoaderClient::createFrame(WTF::String const&, WebCore::HTMLFrameOwnerElement&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebKit/WebProcess/WebCoreSupport/WebFrameLoaderClient.cpp:1595:21
    #10 0x7f7f6f7e6c32 in WebCore::FrameLoader::SubframeLoader::loadSubframe(WebCore::HTMLFrameOwnerElement&, WTF::URL const&, WTF::String const&, WTF::String const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/loader/SubframeLoader.cpp:285:44
    #11 0x7f7f6f7e4570 in WebCore::FrameLoader::SubframeLoader::loadOrRedirectSubframe(WebCore::HTMLFrameOwnerElement&, WTF::URL const&, WTF::AtomString const&, WebCore::LockHistory, WebCore::LockBackForwardList) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/loader/SubframeLoader.cpp:252:17
    #12 0x7f7f6f7e3b57 in WebCore::FrameLoader::SubframeLoader::requestFrame(WebCore::HTMLFrameOwnerElement&, WTF::String const&, WTF::AtomString const&, WebCore::LockHistory, WebCore::LockBackForwardList) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/loader/SubframeLoader.cpp:97:20
    #13 0x7f7f6e9d2e74 in WebCore::HTMLFrameElementBase::openURL(WebCore::LockHistory, WebCore::LockBackForwardList) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/html/HTMLFrameElementBase.cpp:105:44
    #14 0x7f7f6e9d37ce in WebCore::HTMLFrameElementBase::didFinishInsertingNode() /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/html/HTMLFrameElementBase.cpp:145:5
    #15 0x7f7f6df66b9f in void WebCore::executeNodeInsertionWithScriptAssertion<WebCore::ContainerNode::appendChildWithoutPreInsertionValidityCheck(WebCore::Node&)::$_4>(WebCore::ContainerNode&, WebCore::Node&, WebCore::ContainerNode::ChildChange::Source, WebCore::ReplacedAllChildren, WebCore::ContainerNode::appendChildWithoutPreInsertionValidityCheck(WebCore::Node&)::$_4) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:242:17
    #16 0x7f7f6df60bd8 in WebCore::ContainerNode::appendChildWithoutPreInsertionValidityCheck(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:766:9
    #17 0x7f7f6df66222 in WebCore::ContainerNode::appendChild(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/ContainerNode.cpp:732:12
    #18 0x7f7f6e32c36b in WebCore::Node::appendChild(WebCore::Node&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/dom/Node.cpp:511:43
    #19 0x7f7f6ac82972 in WebCore::jsNodePrototypeFunction_appendChildBody(JSC::JSGlobalObject*, JSC::CallFrame*, WebCore::JSNode*)::'lambda'()::operator()() const /path/to/WebKitBuild/GTK/Debug/WebCore/DerivedSources/JSNode.cpp:860:102
    #20 0x7f7f6ac823b3 in void WebCore::invokeFunctorPropagatingExceptionIfNecessary<WebCore::jsNodePrototypeFunction_appendChildBody(JSC::JSGlobalObject*, JSC::CallFrame*, WebCore::JSNode*)::'lambda'()>(JSC::JSGlobalObject&, JSC::ThrowScope&, WebCore::jsNodePrototypeFunction_appendChildBody(JSC::JSGlobalObject*, JSC::CallFrame*, WebCore::JSNode*)::'lambda'()&&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/bindings/js/JSDOMExceptionHandling.h:96:23
    #21 0x7f7f6ac81f42 in WebCore::jsNodePrototypeFunction_appendChildBody(JSC::JSGlobalObject*, JSC::CallFrame*, WebCore::JSNode*) /path/to/WebKitBuild/GTK/Debug/WebCore/DerivedSources/JSNode.cpp:860:5
    #22 0x7f7f6ac81725 in long WebCore::IDLOperation<WebCore::JSNode>::call<&(WebCore::jsNodePrototypeFunction_appendChildBody(JSC::JSGlobalObject*, JSC::CallFrame*, WebCore::JSNode*)), (WebCore::CastedThisErrorBehavior)0>(JSC::JSGlobalObject&, JSC::CallFrame&, char const*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/bindings/js/JSDOMOperation.h:63:9
    #23 0x7f7f6ac74113 in WebCore::jsNodePrototypeFunction_appendChild(JSC::JSGlobalObject*, JSC::CallFrame*) /path/to/WebKitBuild/GTK/Debug/WebCore/DerivedSources/JSNode.cpp:866:12
    #24 0x7f7f1106d1d7  (<unknown module>)
    #25 0x7f7f59dd4ecc in frame_dummy (/path/to/WebKitBuild/GTK/Debug/lib/libjavascriptcoregtk-4.0.so.18+0xf30ecc)
    #26 0x7f7f59dd4ce7 in frame_dummy (/path/to/WebKitBuild/GTK/Debug/lib/libjavascriptcoregtk-4.0.so.18+0xf30ce7)
    #27 0x7f7f59db1ae1 in frame_dummy (/path/to/WebKitBuild/GTK/Debug/lib/libjavascriptcoregtk-4.0.so.18+0xf0dae1)
    #28 0x7f7f5d1b0762 in JSC::JITCode::execute(JSC::VM*, JSC::ProtoCallFrame*) /path/to/WebKitBuild/GTK/Debug/../../../Source/JavaScriptCore/jit/JITCodeInlines.h:42:38
    #29 0x7f7f5d198177 in JSC::Interpreter::executeCall(JSC::JSGlobalObject*, JSC::JSObject*, JSC::CallData const&, JSC::JSValue, JSC::ArgList const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/JavaScriptCore/interpreter/Interpreter.cpp:903:27
    #30 0x7f7f5dac1c4e in JSC::call(JSC::JSGlobalObject*, JSC::JSValue, JSC::CallData const&, JSC::JSValue, JSC::ArgList const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/JavaScriptCore/runtime/CallData.cpp:57:28

SUMMARY: AddressSanitizer: heap-use-after-free /path/to/WebKitBuild/GTK/Debug/WTF/Headers/wtf/RefPtr.h:87:38 in WTF::RefPtr<WTF::WeakPtrImpl<WTF::EmptyCounter>, WTF::RawPtrTraits<WTF::WeakPtrImpl<WTF::EmptyCounter> >, WTF::DefaultRefDerefTraits<WTF::WeakPtrImpl<WTF::EmptyCounter> > >::operator!() const
Shadow bytes around the buggy address:
  0x0c2680019000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 fa
  0x0c2680019010: fa fa fa fa fa fa fa fa 00 00 00 00 00 00 00 00
  0x0c2680019020: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c2680019030: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c2680019040: 00 00 00 00 00 00 00 fa fa fa fa fa fa fa fa fa
=>0x0c2680019050: fd fd fd fd fd fd fd fd fd fd fd[fd]fd fd fd fd
  0x0c2680019060: fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd
  0x0c2680019070: fd fd fd fd fd fd fd fd fd fa fa fa fa fa fa fa
  0x0c2680019080: fa fa fa fa fa fa fa fa 00 00 00 00 00 00 00 00
  0x0c2680019090: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c26800190a0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
  Shadow gap:              cc
==98422==ABORTING
```

# Incorrect memory allocation in WebCore::ImageBufferCairoImageSurfaceBackend::create

Bug 229365

When the [html file](https://github.com/ChijinZ/security_advisories/blob/master/webkitgtk-2.32.3/seeds/incorrect_malloc.html) is opened by webkitgtk, a SEGV is raised by Asan. 

```
=================================================================
==982==ERROR: AddressSanitizer: requested allocation size 0xffffffffbe494e54 (0xffffffffbe495e58 after adjustments for alignment, red zones etc.) exceeds maximum supported size of 0x10000000000 (thread T0)
    #0 0x4c341f in malloc /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:145:3
    #1 0x7fe9971e170b in bmalloc::DebugHeap::malloc(unsigned long, bmalloc::FailureAction) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/DebugHeap.cpp:102:20
    #2 0x7fe9971dc8e7 in bmalloc::Cache::tryAllocateSlowCaseNullCache(bmalloc::HeapKind, unsigned long) /path/to/WebKitBuild/GTK/Debug/../../../Source/bmalloc/bmalloc/Cache.cpp:57:27
    #3 0x7fe996f60e7e in bmalloc::Cache::tryAllocate(bmalloc::HeapKind, unsigned long) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/Cache.h:73:16
    #4 0x7fe996f5f046 in bmalloc::api::tryMalloc(unsigned long, bmalloc::HeapKind) /path/to/WebKitBuild/GTK/Debug/bmalloc/Headers/bmalloc/bmalloc.h:66:12
    #5 0x7fe996f5f046 in WTF::tryFastMalloc(unsigned long) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/FastMalloc.cpp:611:12
    #6 0x7fe996f5ee4b in WTF::tryFastZeroedMalloc(unsigned long) /path/to/WebKitBuild/GTK/Debug/../../../Source/WTF/wtf/FastMalloc.cpp:140:10
    #7 0x7fe9a807af20 in WebCore::ImageBufferCairoImageSurfaceBackend::create(WebCore::ImageBufferBackend::Parameters const&, WebCore::HostWindow const*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/platform/graphics/cairo/ImageBufferCairoImageSurfaceBackend.cpp:80:10
    #8 0x7fe9a7f19bd7 in WTF::RefPtr<WebCore::ConcreteImageBuffer<WebCore::ImageBufferCairoImageSurfaceBackend>, WTF::RawPtrTraits<WebCore::ConcreteImageBuffer<WebCore::ImageBufferCairoImageSurfaceBackend> >, WTF::DefaultRefDerefTraits<WebCore::ConcreteImageBuffer<WebCore::ImageBufferCairoImageSurfaceBackend> > > WebCore::ConcreteImageBuffer<WebCore::ImageBufferCairoImageSurfaceBackend>::create<WebCore::ConcreteImageBuffer<WebCore::ImageBufferCairoImageSurfaceBackend> >(WebCore::FloatSize const&, float, WebCore::DestinationColorSpace const&, WebCore::PixelFormat, WebCore::HostWindow const*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/platform/graphics/ConcreteImageBuffer.h:40:24
    #9 0x7fe9a7f1269b in WebCore::ImageBuffer::create(WebCore::FloatSize const&, WebCore::RenderingMode, float, WebCore::DestinationColorSpace const&, WebCore::PixelFormat, WebCore::HostWindow const*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/platform/graphics/ImageBuffer.cpp:70:23
    #10 0x7fe9a7f137df in WebCore::ImageBuffer::createCompatibleBuffer(WebCore::FloatSize const&, float, WebCore::DestinationColorSpace const&, WebCore::GraphicsContext const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/platform/graphics/ImageBuffer.cpp:116:12
    #11 0x7fe9a7f1349d in WebCore::ImageBuffer::createCompatibleBuffer(WebCore::FloatSize const&, WebCore::DestinationColorSpace const&, WebCore::GraphicsContext const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/platform/graphics/ImageBuffer.cpp:105:24
    #12 0x7fe9a88e6c7f in WebCore::RenderBoxModelObject::paintFillLayerExtended(WebCore::PaintInfo const&, WebCore::Color const&, WebCore::FillLayer const&, WebCore::LayoutRect const&, WebCore::BackgroundBleedAvoidance, WebCore::LegacyInlineFlowBox*, WebCore::LayoutSize const&, WebCore::CompositeOperator, WebCore::RenderElement*, WebCore::BaseBackgroundColorUsage) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBoxModelObject.cpp:876:21
    #13 0x7fe9a88e1c87 in WebCore::RenderBox::paintFillLayer(WebCore::PaintInfo const&, WebCore::Color const&, WebCore::FillLayer const&, WebCore::LayoutRect const&, WebCore::BackgroundBleedAvoidance, WebCore::CompositeOperator, WebCore::RenderElement*, WebCore::BaseBackgroundColorUsage) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBox.cpp:1882:5
    #14 0x7fe9a88c9708 in WebCore::RenderBox::paintFillLayers(WebCore::PaintInfo const&, WebCore::Color const&, WebCore::FillLayer const&, WebCore::LayoutRect const&, WebCore::BackgroundBleedAvoidance, WebCore::CompositeOperator, WebCore::RenderElement*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBox.cpp:1873:9
    #15 0x7fe9a88c8f21 in WebCore::RenderBox::paintRootBoxFillLayers(WebCore::PaintInfo const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBox.cpp:1472:5
    #16 0x7fe9a88d31de in WebCore::RenderBox::paintBackground(WebCore::PaintInfo const&, WebCore::LayoutRect const&, WebCore::BackgroundBleedAvoidance) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBox.cpp:1583:9
    #17 0x7fe9a88cb207 in WebCore::RenderBox::paintBoxDecorations(WebCore::PaintInfo&, WebCore::LayoutPoint const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBox.cpp:1551:9
    #18 0x7fe9a877ae1f in WebCore::RenderBlock::paintObject(WebCore::PaintInfo&, WebCore::LayoutPoint const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBlock.cpp:1253:13
    #19 0x7fe9a8778a53 in WebCore::RenderBlock::paint(WebCore::PaintInfo&, WebCore::LayoutPoint const&) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderBlock.cpp:1130:5
    #20 0x7fe9a8b753cc in WebCore::RenderLayer::paintBackgroundForFragments(WTF::Vector<WebCore::LayerFragment, 1ul, WTF::CrashOnOverflow, 16ul, WTF::FastMalloc> const&, WebCore::GraphicsContext&, WebCore::GraphicsContext&, WebCore::LayoutRect const&, bool, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::PaintBehavior>, WebCore::RenderObject*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3715:20
    #21 0x7fe9a8b671c7 in WebCore::RenderLayer::paintLayerContents(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3359:17
    #22 0x7fe9a8b648fb in WebCore::RenderLayer::paintLayerContentsAndReflection(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3072:5
    #23 0x7fe9a8b60eec in WebCore::RenderLayer::paintLayerWithEffects(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3054:5
    #24 0x7fe9a8b5eaf3 in WebCore::RenderLayer::paintLayer(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:2989:5
    #25 0x7fe9a8b75865 in WebCore::RenderLayer::paintList(WebCore::RenderLayer::LayerList, WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3490:21
    #26 0x7fe9a8b67955 in WebCore::RenderLayer::paintLayerContents(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3386:13
    #27 0x7fe9a8b648fb in WebCore::RenderLayer::paintLayerContentsAndReflection(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3072:5
    #28 0x7fe9a8b60eec in WebCore::RenderLayer::paintLayerWithEffects(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:3054:5
    #29 0x7fe9a8b5eaf3 in WebCore::RenderLayer::paintLayer(WebCore::GraphicsContext&, WebCore::RenderLayer::LayerPaintingInfo const&, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:2989:5
    #30 0x7fe9a8b5e25b in WebCore::RenderLayer::paint(WebCore::GraphicsContext&, WebCore::LayoutRect const&, WebCore::LayoutSize const&, WTF::OptionSet<WebCore::PaintBehavior>, WebCore::RenderObject*, WTF::OptionSet<WebCore::RenderLayer::PaintLayerFlag>, WebCore::RenderLayer::SecurityOriginPaintPolicy, WebCore::EventRegionContext*) /path/to/WebKitBuild/GTK/Debug/../../../Source/WebCore/rendering/RenderLayer.cpp:2858:5

==982==HINT: if you don't care about these errors you may set allocator_may_return_null=1
SUMMARY: AddressSanitizer: allocation-size-too-big /root/llvm/llvm-12/compiler-rt/lib/asan/asan_malloc_linux.cpp:145:3 in malloc
==982==ABORTING

** (MiniBrowser:128556): WARNING **: 08:30:57.017: WebProcess CRASHED
```