

pub struct Payment {

    MerchantID: string,
    MerchantKey: string,
    MerchantSalt: string,
    UserIP: string,
    MerchantOid: string,
    Email: string,
    TotalAmount: int64,
    Currency: string,
    Basket: string,
    NoInstallment: int64,
    MaxInstallment: int64,
    PaytrToken: string,
    Username: string,
    UserAddress: string,
    UserPhone: string,
    OkUrl: string,
    FailUrl: string,
    TestMode: string,
    DebugOn: int8,
    Timeout: int64,
    Lang: string

};

pub struct PaytrResponse {

    Status: int16,
    Token: string,
    Reason: string
};


pub struct CallbackRequest {
    
    InstallmentCount: int16,
    MerchantID: string,
    MerchantOid: string,
    Status: string,
    TotalAmount: int64,
    Hash: string,
    FailReasonCode: int16,
    FailReasonMessage: string,
    TestMode: string,
    PaymentType: string,
    Currency: string,
    PaymentAmount: int64,
};
