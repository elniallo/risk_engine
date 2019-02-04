use std::collections::{HashMap,VecDeque};
use std::time::SystemTime;
use std::error::Error;
use super::account;
#[derive(Clone)]
pub struct ActiveAccount {
    user_id: usize,
    last_active: SystemTime,
}
#[derive(Clone,Deserialize)]
pub struct WithdrawTransaction {
    user_id: usize,
    amount: f64,
    order_type:account::OrderType,
}

impl Default for WithdrawTransaction {
    fn default() -> Self {
        Self {
            user_id: 0,
            amount: 0.0,
            order_type: account::OrderType::USD,
        }
    }
}
#[derive(Debug,Deserialize,Serialize)]
pub struct CompletedTransaction {
    user_id: usize,
    #[serde(rename="bought_quantity")]
    bought_amount: f64,
    #[serde(rename="bought_token")]
    bought_type: account::OrderType,
    #[serde(rename="sold_quantity")]
    sold_amount: f64,
    #[serde(rename="sold_token")]
    sold_type: account::OrderType,
}
#[derive(Serialize)]
pub enum WithdrawalStatus {
    SUFFICIENT_BALANCE,
    INSUFFICIENT_BALANCE,
}

#[derive(Clone,StateData)]
pub struct RiskEngine {
    pending_book: HashMap<usize,Vec<WithdrawTransaction>>,
    recent_accounts: HashMap<usize,account::Account>,
    cache_list: VecDeque<ActiveAccount>,
    DB: HashMap<usize,account::Account>,
}

impl RiskEngine {
    pub fn new()-> Self{
        Self {
            pending_book: HashMap::new(),
            recent_accounts: HashMap::with_capacity(300),
            cache_list: VecDeque::with_capacity(300),
            DB: HashMap::new(),
        }
    }

    pub fn get_account(&mut self,account_id:usize)->Result<&mut account::Account,String>{
        if let Some(account) = self.recent_accounts.get_mut(&account_id) {
            Ok(account)
        } else if let Some(account) = self.DB.get_mut(&account_id) {
            Ok(account)
        } else {
            Err("Account not found".to_string())
        }
    }

    pub fn put_account(&mut self, account: account::Account){
        self.DB.insert(account.get_id(), account);
    }

    pub fn process_withdrawal(&mut self, tx: WithdrawTransaction)->Result<WithdrawalStatus,String>{
        let mut account = self.get_account(tx.user_id)?;
        let valid = account.valid_order(tx.amount, tx.order_type);
        if valid {
            if let Some(pendings) = self.pending_book.get_mut(&tx.user_id) {
                pendings.push(tx);
            } else {
                let insert = self.pending_book.insert(tx.user_id,vec![tx]);
            }
            Ok(WithdrawalStatus::SUFFICIENT_BALANCE)
        } else {
            Ok(WithdrawalStatus::INSUFFICIENT_BALANCE)
        }
    }

    pub fn process_settlement(&mut self, tx:&mut CompletedTransaction) -> Result<(),String> {
        if let Some(pendings) = self.pending_book.get_mut(&tx.user_id) {
            let mut remove = 0;
            for i in 0..pendings.len() {
                if pendings[i].order_type == tx.sold_type && pendings[i].amount >= tx.sold_amount {
                    remove = i;
                    break;
                }
            }
            let removed = pendings.remove(remove);
            if pendings.len() == 0 {
                self.pending_book.remove(&tx.user_id);
            }
            let account = self.get_account(tx.user_id)?;
            account.update(tx.bought_amount,tx.bought_type,removed.amount-tx.sold_amount,tx.sold_type);
        } else {
            return Err("Not found".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn it_inserts_an_account_to_db(){
        let mut engine = RiskEngine::new();
        let account = account::Account::new(100);
        engine.put_account(account);
        let retrieved = engine.get_account(100);
        match retrieved {
            Ok(ac) => {
                assert_eq!(ac.get_id(),100);
            },
            Err(_) => {}
        }
    }

    #[test]
    #[should_panic]
    fn it_should_throw_an_error_if_account_not_found(){
         let mut engine = RiskEngine::new();
        let account = account::Account::new(100);
        engine.put_account(account);
        let retrieved = engine.get_account(150);
        match retrieved {
            Ok(ac) => {
                assert_eq!(ac.get_id(),100);
            },
            Err(_) => {panic!("Should be an error")}
        }
    }
}