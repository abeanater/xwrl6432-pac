#[doc = "Register `ERRSTAT` reader"]
pub type R = crate::R<ErrstatSpec>;
#[doc = "Register `ERRSTAT` writer"]
pub type W = crate::W<ErrstatSpec>;
#[doc = "Field `BUS_ERROR_EVENT` reader - 0:0\\]
Bus Error Event:#br#BUSERR = 0: Condition not detected.#br#BUSERR = 1: TC has detected an error code on the write response bus or read response bus. Error information is stored in Error Details Register \\[ERRDET\\]."]
pub type BusErrorEventR = crate::BitReader;
#[doc = "Field `BUS_ERROR_EVENT` writer - 0:0\\]
Bus Error Event:#br#BUSERR = 0: Condition not detected.#br#BUSERR = 1: TC has detected an error code on the write response bus or read response bus. Error information is stored in Error Details Register \\[ERRDET\\]."]
pub type BusErrorEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_ERROR` reader - 2:2\\]
TR Error:#br#TR detected that violates FIFO Mode transfer \\[SAM or DAM is '1'\\]
alignment rules or has ACNT or BCNT == 0. No additional error information is recorded."]
pub type TrErrorR = crate::BitReader;
#[doc = "Field `TR_ERROR` writer - 2:2\\]
TR Error:#br#TR detected that violates FIFO Mode transfer \\[SAM or DAM is '1'\\]
alignment rules or has ACNT or BCNT == 0. No additional error information is recorded."]
pub type TrErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMR_ADDRESS_ERROR` reader - 3:3\\]
MMR Address Error:#br#MMRAERR = 0 : Condition not detected.#br#MMRAERR = 1 : User attempted to read or write to invalid address configuration memory map. \\[Is only be set for non-emulation accesses\\]. No additional error information is recorded."]
pub type MmrAddressErrorR = crate::BitReader;
#[doc = "Field `MMR_ADDRESS_ERROR` writer - 3:3\\]
MMR Address Error:#br#MMRAERR = 0 : Condition not detected.#br#MMRAERR = 1 : User attempted to read or write to invalid address configuration memory map. \\[Is only be set for non-emulation accesses\\]. No additional error information is recorded."]
pub type MmrAddressErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bus Error Event:#br#BUSERR = 0: Condition not detected.#br#BUSERR = 1: TC has detected an error code on the write response bus or read response bus. Error information is stored in Error Details Register \\[ERRDET\\]."]
    #[inline(always)]
    pub fn bus_error_event(&self) -> BusErrorEventR {
        BusErrorEventR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TR Error:#br#TR detected that violates FIFO Mode transfer \\[SAM or DAM is '1'\\]
alignment rules or has ACNT or BCNT == 0. No additional error information is recorded."]
    #[inline(always)]
    pub fn tr_error(&self) -> TrErrorR {
        TrErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
MMR Address Error:#br#MMRAERR = 0 : Condition not detected.#br#MMRAERR = 1 : User attempted to read or write to invalid address configuration memory map. \\[Is only be set for non-emulation accesses\\]. No additional error information is recorded."]
    #[inline(always)]
    pub fn mmr_address_error(&self) -> MmrAddressErrorR {
        MmrAddressErrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bus Error Event:#br#BUSERR = 0: Condition not detected.#br#BUSERR = 1: TC has detected an error code on the write response bus or read response bus. Error information is stored in Error Details Register \\[ERRDET\\]."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error_event(&mut self) -> BusErrorEventW<ErrstatSpec> {
        BusErrorEventW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TR Error:#br#TR detected that violates FIFO Mode transfer \\[SAM or DAM is '1'\\]
alignment rules or has ACNT or BCNT == 0. No additional error information is recorded."]
    #[inline(always)]
    #[must_use]
    pub fn tr_error(&mut self) -> TrErrorW<ErrstatSpec> {
        TrErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
MMR Address Error:#br#MMRAERR = 0 : Condition not detected.#br#MMRAERR = 1 : User attempted to read or write to invalid address configuration memory map. \\[Is only be set for non-emulation accesses\\]. No additional error information is recorded."]
    #[inline(always)]
    #[must_use]
    pub fn mmr_address_error(&mut self) -> MmrAddressErrorW<ErrstatSpec> {
        MmrAddressErrorW::new(self, 3)
    }
}
#[doc = "Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrstatSpec;
impl crate::RegisterSpec for ErrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errstat::R`](R) reader structure"]
impl crate::Readable for ErrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`errstat::W`](W) writer structure"]
impl crate::Writable for ErrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ErrstatSpec {
    const RESET_VALUE: u32 = 0;
}
