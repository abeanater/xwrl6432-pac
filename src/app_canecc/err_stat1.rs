#[doc = "Register `ERR_STAT1` reader"]
pub type R = crate::R<ErrStat1Spec>;
#[doc = "Register `ERR_STAT1` writer"]
pub type W = crate::W<ErrStat1Spec>;
#[doc = "Field `ECC_SEC` reader - 1:0\\]
TI Internal : Force ECC SEC pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccSecR = crate::FieldReader;
#[doc = "Field `ECC_SEC` writer - 1:0\\]
TI Internal : Force ECC SEC pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_DED` reader - 3:2\\]
TI Internal : Force ECC DED pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccDedR = crate::FieldReader;
#[doc = "Field `ECC_DED` writer - 3:2\\]
TI Internal : Force ECC DED pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_OTHER` reader - 4:4\\]
TI Internal : Force ECC other pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccOtherR = crate::BitReader;
#[doc = "Field `ECC_OTHER` writer - 4:4\\]
TI Internal : Force ECC other pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_PAR` reader - 6:5\\]
TI Internal : Force ECC parity pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccParR = crate::FieldReader;
#[doc = "Field `ECC_PAR` writer - 6:5\\]
TI Internal : Force ECC parity pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccParW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_CTRL_REG` reader - 7:7\\]
TI Internal : Force ctrl reg pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccCtrlRegR = crate::BitReader;
#[doc = "Field `ECC_CTRL_REG` writer - 7:7\\]
TI Internal : Force ctrl reg pending interrupt. Write 1 to set. This bit is self clearing."]
pub type EccCtrlRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ECC_SEC` reader - 9:8\\]
TI Internal : Clear Single Bit Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccSecR = crate::FieldReader;
#[doc = "Field `CLR_ECC_SEC` writer - 9:8\\]
TI Internal : Clear Single Bit Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_ECC_DED` reader - 11:10\\]
TI Internal : Clear Double Bit Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccDedR = crate::FieldReader;
#[doc = "Field `CLR_ECC_DED` writer - 11:10\\]
TI Internal : Clear Double Bit Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccDedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_ECC_OTHER` reader - 12:12\\]
TI Internal : Clear Other Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccOtherR = crate::BitReader;
#[doc = "Field `CLR_ECC_OTHER` writer - 12:12\\]
TI Internal : Clear Other Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccOtherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ECC_PAR` reader - 14:13\\]
TI Internal : Clear Parity Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccParR = crate::FieldReader;
#[doc = "Field `CLR_ECC_PAR` writer - 14:13\\]
TI Internal : Clear Parity Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccParW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_ECC_CTRL_REG` reader - 15:15\\]
TI Internal : Clear Ctrl Reg Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccCtrlRegR = crate::BitReader;
#[doc = "Field `CLR_ECC_CTRL_REG` writer - 15:15\\]
TI Internal : Clear Ctrl Reg Error Status. Write 1 to clear. This bit is self clearing."]
pub type ClrEccCtrlRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_BIT1_STS` reader - 31:16\\]
TI Internal : Data bit that corresponds to the single-bit error"]
pub type EccBit1StsR = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT1_STS` writer - 31:16\\]
TI Internal : Data bit that corresponds to the single-bit error"]
pub type EccBit1StsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
TI Internal : Force ECC SEC pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    pub fn ecc_sec(&self) -> EccSecR {
        EccSecR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
TI Internal : Force ECC DED pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    pub fn ecc_ded(&self) -> EccDedR {
        EccDedR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal : Force ECC other pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    pub fn ecc_other(&self) -> EccOtherR {
        EccOtherR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
TI Internal : Force ECC parity pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    pub fn ecc_par(&self) -> EccParR {
        EccParR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal : Force ctrl reg pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    pub fn ecc_ctrl_reg(&self) -> EccCtrlRegR {
        EccCtrlRegR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal : Clear Single Bit Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_ecc_sec(&self) -> ClrEccSecR {
        ClrEccSecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
TI Internal : Clear Double Bit Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_ecc_ded(&self) -> ClrEccDedR {
        ClrEccDedR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
TI Internal : Clear Other Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_ecc_other(&self) -> ClrEccOtherR {
        ClrEccOtherR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
TI Internal : Clear Parity Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_ecc_par(&self) -> ClrEccParR {
        ClrEccParR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
TI Internal : Clear Ctrl Reg Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_ecc_ctrl_reg(&self) -> ClrEccCtrlRegR {
        ClrEccCtrlRegR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
TI Internal : Data bit that corresponds to the single-bit error"]
    #[inline(always)]
    pub fn ecc_bit1_sts(&self) -> EccBit1StsR {
        EccBit1StsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
TI Internal : Force ECC SEC pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_sec(&mut self) -> EccSecW<ErrStat1Spec> {
        EccSecW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
TI Internal : Force ECC DED pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_ded(&mut self) -> EccDedW<ErrStat1Spec> {
        EccDedW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal : Force ECC other pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_other(&mut self) -> EccOtherW<ErrStat1Spec> {
        EccOtherW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
TI Internal : Force ECC parity pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_par(&mut self) -> EccParW<ErrStat1Spec> {
        EccParW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal : Force ctrl reg pending interrupt. Write 1 to set. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_ctrl_reg(&mut self) -> EccCtrlRegW<ErrStat1Spec> {
        EccCtrlRegW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal : Clear Single Bit Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_sec(&mut self) -> ClrEccSecW<ErrStat1Spec> {
        ClrEccSecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
TI Internal : Clear Double Bit Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_ded(&mut self) -> ClrEccDedW<ErrStat1Spec> {
        ClrEccDedW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
TI Internal : Clear Other Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_other(&mut self) -> ClrEccOtherW<ErrStat1Spec> {
        ClrEccOtherW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
TI Internal : Clear Parity Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_par(&mut self) -> ClrEccParW<ErrStat1Spec> {
        ClrEccParW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
TI Internal : Clear Ctrl Reg Error Status. Write 1 to clear. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn clr_ecc_ctrl_reg(&mut self) -> ClrEccCtrlRegW<ErrStat1Spec> {
        ClrEccCtrlRegW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
TI Internal : Data bit that corresponds to the single-bit error"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_bit1_sts(&mut self) -> EccBit1StsW<ErrStat1Spec> {
        EccBit1StsW::new(self, 16)
    }
}
#[doc = "ERR_STAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat1Spec;
impl crate::RegisterSpec for ErrStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat1::R`](R) reader structure"]
impl crate::Readable for ErrStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat1::W`](W) writer structure"]
impl crate::Writable for ErrStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_STAT1 to value 0"]
impl crate::Resettable for ErrStat1Spec {
    const RESET_VALUE: u32 = 0;
}
