use std::cmp::{PartialOrd,Ordering};
#[derive(PartialEq,Clone,StateData,Deserialize)]
pub struct Account {
    #[serde(rename="User_Id")]
    id: usize,
     #[serde(rename="USD")]
    usd: f64,
     #[serde(rename="EUR")]
    eur: f64,
     #[serde(rename="BTC")]
    btc: f64,
     #[serde(rename="BCH")]
    bch: f64,
     #[serde(rename="ETH")]
    eth: f64,
}
#[derive(Copy,Clone, Debug,Deserialize,PartialEq,Serialize)]
pub enum OrderType {
    USD,
    EUR,
    BTC,
    BCH,
    ETH,
}


impl Account {
    pub fn new(id:usize) -> Self{
        Self {id,usd:0.0,eur:0.0,btc:0.0,bch:0.0,eth:0.0}
    }

    pub fn get_id(&self)->usize {
        self.id
    }

    pub fn update(&mut self, buy_amt:f64,buy_type:OrderType,sell_adjustment:f64,sell_type:OrderType){
        match buy_type {
            OrderType::USD => {
                self.usd += buy_amt      
            },
            OrderType::EUR => {
                self.eur+= buy_amt
            },
            OrderType::BTC => {
                self.btc+= buy_amt
            },
            OrderType::BCH => {
                self.bch+=buy_amt
            },            
            OrderType::ETH => {
                self.eth+=buy_amt
            }
        }
        match sell_type {
            OrderType::USD => {
                self.usd += sell_adjustment      
            },
            OrderType::EUR => {
                self.eur+= sell_adjustment
            },
            OrderType::BTC => {
                self.btc+= sell_adjustment
            },
            OrderType::BCH => {
                self.bch+=sell_adjustment
            },            
            OrderType::ETH => {
                self.eth+=sell_adjustment
            }
        }
    }

    pub fn usd_order(&mut self, amount: f64){
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

    pub fn eth_order(&mut self, amount:f64){
        self.eth += amount;
    }

    pub fn get_balances(&self) -> Vec<f64>{
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