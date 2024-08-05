#[doc = "Register `ERROR_STATUS1` reader"]
pub type R = crate::R<ErrorStatus1Spec>;
#[doc = "Register `ERROR_STATUS1` writer"]
pub type W = crate::W<ErrorStatus1Spec>;
#[doc = "Field `ECC_SEC` reader - 1:0\\]
Level Single Bit Error Status - (RW incr)"]
pub type EccSecR = crate::FieldReader;
#[doc = "Field `ECC_SEC` writer - 1:0\\]
Level Single Bit Error Status - (RW incr)"]
pub type EccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_DED` reader - 3:2\\]
Level Double Bit Error Status - (RW incr)"]
pub type EccDedR = crate::FieldReader;
#[doc = "Field `ECC_DED` writer - 3:2\\]
Level Double Bit Error Status - (RW incr)"]
pub type EccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_OTHER` reader - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending Level interrupt - (RW"]
pub type EccOtherR = crate::BitReader;
#[doc = "Field `ECC_OTHER` writer - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending Level interrupt - (RW"]
pub type EccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR` reader - 6:5\\]
Level parity error Error Status - (RW )"]
pub type ParityErrR = crate::FieldReader;
#[doc = "Field `PARITY_ERR` writer - 6:5\\]
Level parity error Error Status - (RW )"]
pub type ParityErrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTR_REG_ERR` reader - 7:7\\]
control register error pending Level interrupt - (RW"]
pub type CtrRegErrR = crate::BitReader;
#[doc = "Field `CTR_REG_ERR` writer - 7:7\\]
control register error pending Level interrupt - (RW"]
pub type CtrRegErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ECC_SEC` reader - 9:8\\]
Clear Single Bit Error Status - (RW decr)"]
pub type ClrEccSecR = crate::FieldReader;
#[doc = "Field `CLR_ECC_SEC` writer - 9:8\\]
Clear Single Bit Error Status - (RW decr)"]
pub type ClrEccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_ECC_DED` reader - 11:10\\]
Clear Double Bit Error Status - (RW decr)"]
pub type ClrEccDedR = crate::FieldReader;
#[doc = "Field `CLR_ECC_DED` writer - 11:10\\]
Clear Double Bit Error Status - (RW decr)"]
pub type ClrEccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_ECC_OTHER` reader - 12:12\\]
Clear other Error Status - (RW )"]
pub type ClrEccOtherR = crate::BitReader;
#[doc = "Field `CLR_ECC_OTHER` writer - 12:12\\]
Clear other Error Status - (RW )"]
pub type ClrEccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PARITY_ERR` reader - 14:13\\]
Clear parity Error Status - (RW decr)"]
pub type ClrParityErrR = crate::FieldReader;
#[doc = "Field `CLR_PARITY_ERR` writer - 14:13\\]
Clear parity Error Status - (RW decr)"]
pub type ClrParityErrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_CTRL_REG_ERR` reader - 15:15\\]
Clear control reg error Error Status you must also re write the contorl ergister itself to clear this - (RW )"]
pub type ClrCtrlRegErrR = crate::BitReader;
#[doc = "Field `CLR_CTRL_REG_ERR` writer - 15:15\\]
Clear control reg error Error Status you must also re write the contorl ergister itself to clear this - (RW )"]
pub type ClrCtrlRegErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_BIT1` reader - 31:16\\]
Data bit that corresponds to the single-bit error - (RO )"]
pub type EccBit1R = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT1` writer - 31:16\\]
Data bit that corresponds to the single-bit error - (RO )"]
pub type EccBit1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Level Single Bit Error Status - (RW incr)"]
    #[inline(always)]
    pub fn ecc_sec(&self) -> EccSecR {
        EccSecR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Level Double Bit Error Status - (RW incr)"]
    #[inline(always)]
    pub fn ecc_ded(&self) -> EccDedR {
        EccDedR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending Level interrupt - (RW"]
    #[inline(always)]
    pub fn ecc_other(&self) -> EccOtherR {
        EccOtherR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Level parity error Error Status - (RW )"]
    #[inline(always)]
    pub fn parity_err(&self) -> ParityErrR {
        ParityErrR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
control register error pending Level interrupt - (RW"]
    #[inline(always)]
    pub fn ctr_reg_err(&self) -> CtrRegErrR {
        CtrRegErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clear Single Bit Error Status - (RW decr)"]
    #[inline(always)]
    pub fn clr_ecc_sec(&self) -> ClrEccSecR {
        ClrEccSecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Clear Double Bit Error Status - (RW decr)"]
    #[inline(always)]
    pub fn clr_ecc_ded(&self) -> ClrEccDedR {
        ClrEccDedR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear other Error Status - (RW )"]
    #[inline(always)]
    pub fn clr_ecc_other(&self) -> ClrEccOtherR {
        ClrEccOtherR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Clear parity Error Status - (RW decr)"]
    #[inline(always)]
    pub fn clr_parity_err(&self) -> ClrParityErrR {
        ClrParityErrR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Clear control reg error Error Status you must also re write the contorl ergister itself to clear this - (RW )"]
    #[inline(always)]
    pub fn clr_ctrl_reg_err(&self) -> ClrCtrlRegErrR {
        ClrCtrlRegErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that corresponds to the single-bit error - (RO )"]
    #[inline(always)]
    pub fn ecc_bit1(&self) -> EccBit1R {
        EccBit1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Level Single Bit Error Status - (RW incr)"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_sec(&mut self) -> EccSecW<ErrorStatus1Spec> {
        EccSecW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Level Double Bit Error Status - (RW incr)"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_ded(&mut self) -> EccDedW<ErrorStatus1Spec> {
        EccDedW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending Level interrupt - (RW"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_other(&mut self) -> EccOtherW<ErrorStatus1Spec> {
        EccOtherW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Level parity error Error Status - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn parity_err(&mut self) -> ParityErrW<ErrorStatus1Spec> {
        ParityErrW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
control register error pending Level interrupt - (RW"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_reg_err(&mut self) -> CtrRegErrW<ErrorStatus1Spec> {
        CtrRegErrW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clear Single Bit Error Status - (RW decr)"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_sec(&mut self) -> ClrEccSecW<ErrorStatus1Spec> {
        ClrEccSecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Clear Double Bit Error Status - (RW decr)"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_ded(&mut self) -> ClrEccDedW<ErrorStatus1Spec> {
        ClrEccDedW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear other Error Status - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_other(&mut self) -> ClrEccOtherW<ErrorStatus1Spec> {
        ClrEccOtherW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Clear parity Error Status - (RW decr)"]
    #[inline(always)]
    #[must_use]
    pub fn clr_parity_err(&mut self) -> ClrParityErrW<ErrorStatus1Spec> {
        ClrParityErrW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Clear control reg error Error Status you must also re write the contorl ergister itself to clear this - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ctrl_reg_err(&mut self) -> ClrCtrlRegErrW<ErrorStatus1Spec> {
        ClrCtrlRegErrW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that corresponds to the single-bit error - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_bit1(&mut self) -> EccBit1W<ErrorStatus1Spec> {
        EccBit1W::new(self, 16)
    }
}
#[doc = "ECC Error Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatus1Spec;
impl crate::RegisterSpec for ErrorStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status1::R`](R) reader structure"]
impl crate::Readable for ErrorStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`error_status1::W`](W) writer structure"]
impl crate::Writable for ErrorStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS1 to value 0"]
impl crate::Resettable for ErrorStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
