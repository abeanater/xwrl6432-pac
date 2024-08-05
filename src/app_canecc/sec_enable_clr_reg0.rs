#[doc = "Register `SEC_ENABLE_CLR_REG0` reader"]
pub type R = crate::R<SecEnableClrReg0Spec>;
#[doc = "Register `SEC_ENABLE_CLR_REG0` writer"]
pub type W = crate::W<SecEnableClrReg0Spec>;
#[doc = "Field `SEC_EN_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type SecEnClrR = crate::BitReader;
#[doc = "Field `SEC_EN_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type SecEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type CtrlEdcVbussEnableClrR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
pub type CtrlEdcVbussEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU10` reader - 31:2\\]
Reserved"]
pub type Nu10R = crate::FieldReader<u32>;
#[doc = "Field `NU10` writer - 31:2\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    pub fn sec_en_clr(&self) -> SecEnClrR {
        SecEnClrR::new((self.bits & 1) != 0)
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
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn sec_en_clr(&mut self) -> SecEnClrW<SecEnableClrReg0Spec> {
        SecEnClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field. Reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_enable_clr(&mut self) -> CtrlEdcVbussEnableClrW<SecEnableClrReg0Spec> {
        CtrlEdcVbussEnableClrW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<SecEnableClrReg0Spec> {
        Nu10W::new(self, 2)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableClrReg0Spec;
impl crate::RegisterSpec for SecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for SecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for SecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_ENABLE_CLR_REG0 to value 0"]
impl crate::Resettable for SecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
