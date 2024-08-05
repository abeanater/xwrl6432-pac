#[doc = "Register `DED_ENABLE_CLR_REG0` reader"]
pub type R = crate::R<DedEnableClrReg0Spec>;
#[doc = "Register `DED_ENABLE_CLR_REG0` writer"]
pub type W = crate::W<DedEnableClrReg0Spec>;
#[doc = "Field `DED_EN_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type DedEnClrR = crate::BitReader;
#[doc = "Field `DED_EN_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type DedEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type CtrlEdcVbussEnableClrR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type CtrlEdcVbussEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU14` reader - 31:2\\]
Reserved"]
pub type Nu14R = crate::FieldReader<u32>;
#[doc = "Field `NU14` writer - 31:2\\]
Reserved"]
pub type Nu14W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    pub fn ded_en_clr(&self) -> DedEnClrR {
        DedEnClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    pub fn ctrl_edc_vbuss_enable_clr(&self) -> CtrlEdcVbussEnableClrR {
        CtrlEdcVbussEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu14(&self) -> Nu14R {
        Nu14R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn ded_en_clr(&mut self) -> DedEnClrW<DedEnableClrReg0Spec> {
        DedEnClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_enable_clr(&mut self) -> CtrlEdcVbussEnableClrW<DedEnableClrReg0Spec> {
        CtrlEdcVbussEnableClrW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu14(&mut self) -> Nu14W<DedEnableClrReg0Spec> {
        Nu14W::new(self, 2)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableClrReg0Spec;
impl crate::RegisterSpec for DedEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for DedEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for DedEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_ENABLE_CLR_REG0 to value 0"]
impl crate::Resettable for DedEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
