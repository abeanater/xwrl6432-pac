#[doc = "Register `DED_ENABLE_SET_REG0` reader"]
pub type R = crate::R<DedEnableSetReg0Spec>;
#[doc = "Register `DED_ENABLE_SET_REG0` writer"]
pub type W = crate::W<DedEnableSetReg0Spec>;
#[doc = "Field `DED_EN_SET` reader - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type DedEnSetR = crate::BitReader;
#[doc = "Field `DED_EN_SET` writer - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type DedEnSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type CtrlEdcVbussEnableSetR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type CtrlEdcVbussEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU13` reader - 31:2\\]
Reserved"]
pub type Nu13R = crate::FieldReader<u32>;
#[doc = "Field `NU13` writer - 31:2\\]
Reserved"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn ded_en_set(&self) -> DedEnSetR {
        DedEnSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn ctrl_edc_vbuss_enable_set(&self) -> CtrlEdcVbussEnableSetR {
        CtrlEdcVbussEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn ded_en_set(&mut self) -> DedEnSetW<DedEnableSetReg0Spec> {
        DedEnSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_enable_set(&mut self) -> CtrlEdcVbussEnableSetW<DedEnableSetReg0Spec> {
        CtrlEdcVbussEnableSetW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<DedEnableSetReg0Spec> {
        Nu13W::new(self, 2)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableSetReg0Spec;
impl crate::RegisterSpec for DedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for DedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for DedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_ENABLE_SET_REG0 to value 0"]
impl crate::Resettable for DedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
