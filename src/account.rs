use std::cmp::{PartialOrd,Ordering};
#[derive(PartialEq)]
pub struct Account {
    id: usize,
    usd: f64,
    eur: f64,
    btc: f64,
    bch: f64,
    eth: f64,
    last_active: usize,
}
#[derive(Copy,Clone, Deserialize)]
pub enum OrderType {
    USD,
    EUR,
    BTC,
    BCH,
    ETH,
}

impl Account {
    pub fn new(id:usize) -> Self{
        Self {id,usd:0.0,eur:0.0,btc:0.0,bch:0.0,eth:0.0,last_active:0}
    }

    pub fn get_id(&self)->usize {
        self.id
    }

    fn usd_order(&mut self, amount: f64){
        self.usd += amount;
    }

    fn eur_order(&mut self, amount:f64){
        self.eur += amount;
    }

    fn btc_order(&mut self, amount:f64){
        self.btc += amount;
    }

    fn bch_order(&mut self, amount:f64){
        self.bch += amount;
    }

    fn eth_order(&mut self, amount:f64){
        self.eth += amount;
    }

    fn get_balances(&self) -> Vec<f64>{
        vec![self.usd,self.eur,self.btc,self.bch,self.eth]
    }

    pub fn valid_order(&mut self, amount:f64,order_type:OrderType) -> bool {
        match order_type {
            OrderType::USD => {
                if self.usd >= amount {
                    self.usd_order(-amount);
                    return true;
                } else {
                    return false;
                }
            },
            OrderType::EUR => {
                if self.eur >= amount {
                    self.eur_order(-amount);
                    return true;
                } else {
                    return false;
                }
            },
            OrderType::BTC => {
                if self.btc >= amount {
                    self.btc_order(-amount);
                    return true;
                } else {
                    return false;
                }
            },
            OrderType::BCH => {
                if self.bch >= amount {
                    self.bch_order(-amount);
                    return true;
                } else {
                    return false;
                }
            },            
            OrderType::ETH => {
                if self.eth >= amount {
                    self.eth_order(-amount);
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other:&Account)->Option<Ordering>{
        if self.id == other.id {
            Some(Ordering::Equal)
        } else if self.id < other.id {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn it_initialises_an_account_with_zero_balance(){
        let mut account = Account::new(100);
        assert_eq!(account.get_balances(),vec![0.0,0.0,0.0,0.0,0.0]);
    }

    #[test]
    fn it_adjusts_the_balance_of_currencies_up_and_down(){
        let mut account = Account::new(100);
        account.usd_order(23.5);
        account.eth_order(1.12344);
        assert_eq!(account.get_balances(),vec![23.5,0.0,0.0,0.0,1.12344]);
        account.usd_order(-23.5);
        account.eth_order(-1.12344);
        assert_eq!(account.get_balances(),vec![0.0,0.0,0.0,0.0,0.0]);
    }

    #[test]
    fn it_should_not_permit_transactions_where_spendable_balance_does_not_exist(){
        let mut account = Account::new(100);
        account.usd_order(23.5);
        account.eth_order(1.12344);
        let valid = account.valid_order(22.0, OrderType::USD);
        let invalid = account.valid_order(1.5, OrderType::ETH);
        assert_eq!(valid,true);
        assert_eq!(invalid,false);
        assert_eq!(account.get_balances(),vec![1.5,0.0,0.0,0.0,1.12344]);
    }
}