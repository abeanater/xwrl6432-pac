#[doc = "Register `SEC_ENABLE_SET_REG0` reader"]
pub type R = crate::R<SecEnableSetReg0Spec>;
#[doc = "Register `SEC_ENABLE_SET_REG0` writer"]
pub type W = crate::W<SecEnableSetReg0Spec>;
#[doc = "Field `SEC_EN_SET` reader - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type SecEnSetR = crate::BitReader;
#[doc = "Field `SEC_EN_SET` writer - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type SecEnSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type CtrlEdcVbussEnableSetR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type CtrlEdcVbussEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU9` reader - 31:2\\]
Reserved"]
pub type Nu9R = crate::FieldReader<u32>;
#[doc = "Field `NU9` writer - 31:2\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn sec_en_set(&self) -> SecEnSetR {
        SecEnSetR::new((self.bits & 1) != 0)
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
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for msgmem_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn sec_en_set(&mut self) -> SecEnSetW<SecEnableSetReg0Spec> {
        SecEnSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_enable_set(&mut self) -> CtrlEdcVbussEnableSetW<SecEnableSetReg0Spec> {
        CtrlEdcVbussEnableSetW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<SecEnableSetReg0Spec> {
        Nu9W::new(self, 2)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableSetReg0Spec;
impl crate::RegisterSpec for SecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for SecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for SecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_ENABLE_SET_REG0 to value 0"]
impl crate::Resettable for SecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
