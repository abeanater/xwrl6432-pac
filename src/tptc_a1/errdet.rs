#[doc = "Register `ERRDET` reader"]
pub type R = crate::R<ErrdetSpec>;
#[doc = "Register `ERRDET` writer"]
pub type W = crate::W<ErrdetSpec>;
#[doc = "Field `TRANSACTION_STATUS` reader - 3:0\\]
Transaction Status:#br#Stores the non-zero status/error code that was detected on the read status or write status bus.#br#MS-bit effectively serves as the read vs. write error code.#br#If read status and write status are returned on the same cycle then the TC chooses non-zero version. If both are non-zero then write status is treated as higher priority.#br#Encoding of errors matches the CBA spec."]
pub type TransactionStatusR = crate::FieldReader;
#[doc = "Field `TRANSACTION_STATUS` writer - 3:0\\]
Transaction Status:#br#Stores the non-zero status/error code that was detected on the read status or write status bus.#br#MS-bit effectively serves as the read vs. write error code.#br#If read status and write status are returned on the same cycle then the TC chooses non-zero version. If both are non-zero then write status is treated as higher priority.#br#Encoding of errors matches the CBA spec."]
pub type TransactionStatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRANSFER_COMPLETE_CODE` reader - 13:8\\]
Transfer Complete Code: Contains the OPT.TCC value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type TransferCompleteCodeR = crate::FieldReader;
#[doc = "Field `TRANSFER_COMPLETE_CODE` writer - 13:8\\]
Transfer Complete Code: Contains the OPT.TCC value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type TransferCompleteCodeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CONTAINS_THE_OPT_TCINTEN` reader - 16:16\\]
Contains the OPT.TCINTEN value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type ContainsTheOptTcintenR = crate::BitReader;
#[doc = "Field `CONTAINS_THE_OPT_TCINTEN` writer - 16:16\\]
Contains the OPT.TCINTEN value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type ContainsTheOptTcintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTAINS_THE_OPT_TCCHEN` reader - 17:17\\]
Contains the OPT.TCCHEN value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type ContainsTheOptTcchenR = crate::BitReader;
#[doc = "Field `CONTAINS_THE_OPT_TCCHEN` writer - 17:17\\]
Contains the OPT.TCCHEN value programmed by the user for the Read or Write transaction that resulted in an error."]
pub type ContainsTheOptTcchenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Transaction Status:#br#Stores the non-zero status/error code that was detected on the read status or write status bus.#br#MS-bit effectively serves as the read vs. write error code.#br#If read status and write status are returned on the same cycle then the TC chooses non-zero version. If both are non-zero then write status is treated as higher priority.#br#Encoding of errors matches the CBA spec."]
    #[inline(always)]
    pub fn transaction_status(&self) -> TransactionStatusR {
        TransactionStatusR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Transfer Complete Code: Contains the OPT.TCC value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    pub fn transfer_complete_code(&self) -> TransferCompleteCodeR {
        TransferCompleteCodeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Contains the OPT.TCINTEN value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    pub fn contains_the_opt_tcinten(&self) -> ContainsTheOptTcintenR {
        ContainsTheOptTcintenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Contains the OPT.TCCHEN value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    pub fn contains_the_opt_tcchen(&self) -> ContainsTheOptTcchenR {
        ContainsTheOptTcchenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Transaction Status:#br#Stores the non-zero status/error code that was detected on the read status or write status bus.#br#MS-bit effectively serves as the read vs. write error code.#br#If read status and write status are returned on the same cycle then the TC chooses non-zero version. If both are non-zero then write status is treated as higher priority.#br#Encoding of errors matches the CBA spec."]
    #[inline(always)]
    #[must_use]
    pub fn transaction_status(&mut self) -> TransactionStatusW<ErrdetSpec> {
        TransactionStatusW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Transfer Complete Code: Contains the OPT.TCC value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_code(&mut self) -> TransferCompleteCodeW<ErrdetSpec> {
        TransferCompleteCodeW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Contains the OPT.TCINTEN value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    #[must_use]
    pub fn contains_the_opt_tcinten(&mut self) -> ContainsTheOptTcintenW<ErrdetSpec> {
        ContainsTheOptTcintenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Contains the OPT.TCCHEN value programmed by the user for the Read or Write transaction that resulted in an error."]
    #[inline(always)]
    #[must_use]
    pub fn contains_the_opt_tcchen(&mut self) -> ContainsTheOptTcchenW<ErrdetSpec> {
        ContainsTheOptTcchenW::new(self, 17)
    }
}
#[doc = "Error Details Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errdet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errdet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrdetSpec;
impl crate::RegisterSpec for ErrdetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errdet::R`](R) reader structure"]
impl crate::Readable for ErrdetSpec {}
#[doc = "`write(|w| ..)` method takes [`errdet::W`](W) writer structure"]
impl crate::Writable for ErrdetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRDET to value 0"]
impl crate::Resettable for ErrdetSpec {
    const RESET_VALUE: u32 = 0;
}
