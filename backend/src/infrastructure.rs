// --- インメモリリポジトリの実装 ---

/* --- 古い商品リポジトリ実装をコメントアウト --- */
/*
#[derive(Clone, Default)]
pub struct InMemory商品Repository {
    items: Arc<Mutex<HashMap<商品ID, 商品>>>,
}

impl InMemory商品Repository {
     pub fn new() -> Self {
         let mut items_map = HashMap::new();
         let item1 = 商品 {
             id: 商品ID::new(),
             名前: "すごい本".to_string(),
             価格: 3000,
         };
         let item2 = 商品 {
             id: 商品ID::new(),
             名前: "便利なツール".to_string(),
             価格: 5000,
         };
         items_map.insert(item1.id, item1);
         items_map.insert(item2.id, item2);

         Self { items: Arc::new(Mutex::new(items_map)) }
     }

     #[allow(dead_code)]
     pub fn get_sample_item_ids(&self) -> Vec<商品ID> {
         self.items.lock().unwrap().keys().cloned().collect()
     }
}

impl 商品Repository for InMemory商品Repository {
    fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError> {
        let items_map = self.items.lock().unwrap();
        Ok(items_map.get(id).cloned())
    }
}
*/

/* --- 古い注文リポジトリ実装をコメントアウト --- */
/*
#[derive(Clone, Default)]
pub struct InMemory注文Repository {
    orders: Arc<Mutex<HashMap<注文ID, 注文>>>,
}

 impl InMemory注文Repository {
    pub fn new() -> Self {
        Self { orders: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl 注文Repository for InMemory注文Repository {
    fn save(&self, order: &注文) -> Result<(), DomainError> {
        let mut orders_map = self.orders.lock().unwrap();
        println!("Saving order: {:?}", order); // 永続化処理のシミュレーション
        orders_map.insert(order.id, order.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &注文ID) -> Result<Option<注文>, DomainError> {
        let orders_map = self.orders.lock().unwrap();
        println!("Finding order by id: {:?}", id);
        Ok(orders_map.get(id).cloned())
    }
}
*/ 