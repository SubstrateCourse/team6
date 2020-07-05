# 作业

## 补完剩下的代码
答：`./pallets/kitties/src/linked_item.rs`

## 修复单元测试
答：`./pallets/kitties/src/lib.rs`

## 阅读 pallet-membership
* 分析 add_member 的计算复杂度
答：
``` Rust
/// 总计算复杂度: O(MP + logN)
#[weight = 50_000_000]
pub fn add_member(origin, who: T::AccountId) {

	// O(1) 
	T::AddOrigin::ensure_origin(origin)?;

	// 一次 DB 读取，编码解码: O(n)
	let mut members = <Members<T, I>>::get();

	// O(log(n)) 
	let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;

	// O(n)
	members.insert(location, who.clone());

	// 一次 DB 读取，编码解码: O(n)
	<Members<T, I>>::put(&members);

	// frame/collective/src/lib.rs:848 `fn change_members_sorted(...){...}`
	// - `O(MP + N)`
	//   - where `M` old-members-count (governance-bounded)
	//   - where `N` new-members-count (governance-bounded)
	//   - where `P` proposals-count
	T::MembershipChanged::change_members_sorted(&[who], &[], &members[..]);
	
	// 参考 substrate/frame/system/src/lib.rs :
	// pub fn deposit_event_indexed(topics: &[T::Hash], event: T::Event) {...}
	// Self::block_number(); 一次 DB 读取，编码解码: O(n)
	// ExecutionPhase::get(); 一次 DB 读取，编码解码: O(n)
	// EventCount::get(); 一次 DB 读取，编码解码: O(n)
	// EventCount::put(new_event_count); 一次 DB 写入，编码解码: O(n)
	// Events::<T>::append(&event); 一次 DB 写入，编码解码: O(n)
	Self::deposit_event(RawEvent::MemberAdded);
}
```

* 分析 pallet-membership 是否适合以下场景下使用,提供原因
  * 储存预言机提供者
答：适合。人数不多, 并且增删改不频繁。

  * 储存游戏链中每个工会的成员
答：不适合。add_member 计算复杂度较高，对于人数太多, 关系复杂，并且增删改频繁的游戏工会成员场景，成本较大，所以不适合。

  * 储存 PoA 网络验证人
答：适合。人数不多, PoA 验证人相对固定, 并且增删改不频繁。
