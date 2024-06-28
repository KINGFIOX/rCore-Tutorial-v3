# README

## map

map 就是添加 (vpn, ppn)

### page_table

page_table 的 map 添加 (vpn, ppn) , 而 ppn 是存储在 pte 中的,
pte 又会有 flags 。 因此 page_table 的 map 的函数签名是: `pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags)`

```rust
#[allow(unused)]
pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
  let pte = self.find_pte_create(vpn).unwrap();
  assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
  *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
}
```

### map_area

MapArea 的插入 与 page_table 的插入应该要同步。

```rust
pub struct MapArea {
  vpn_range: VPNRange,
  /// (virtual page number, frame 里面有 physical page number )
  data_frames: BTreeMap<VirtPageNum, FrameTracker>,
  map_type: MapType,
  map_perm: MapPermission,
}
```

```rust
pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
  let ppn: PhysPageNum;
  match self.map_type {
    MapType::Identical => {
      ppn = PhysPageNum(vpn.0);
    }
    MapType::Framed => {
      // 分配一个 ppn
      let frame = frame_alloc().unwrap();
      ppn = frame.ppn;
      // BTreeMap 中插入: vpn 与 ppn 的映射
      self.data_frames.insert(vpn, frame);
    }
  }
  let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();

  // page_table 上设置 pte
  page_table.map(vpn, ppn, pte_flags);
}
```

```rust
pub fn map(&mut self, page_table: &mut PageTable) {
  for vpn in self.vpn_range {
    self.map_one(page_table, vpn);
  }
}
```

### memory_set

```rust
pub struct MemorySet {
  page_table: PageTable,
  areas: Vec<MapArea>,
}
```

因此 memory_set 的 map 是这样的:

```rust
fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
  map_area.map(&mut self.page_table);
  if let Some(data) = data {
    map_area.copy_data(&mut self.page_table, data);
  }
  self.areas.push(map_area);
}
```

做了几件事:
