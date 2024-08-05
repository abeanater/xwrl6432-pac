#[doc = "Register `SEC_STATUS_REG0` reader"]
pub type R = crate::R<SecStatusReg0Spec>;
#[doc = "Register `SEC_STATUS_REG0` writer"]
pub type W = crate::W<SecStatusReg0Spec>;
#[doc = "Field `SEC_PEND` reader - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
pub type SecPendR = crate::BitReader;
#[doc = "Field `SEC_PEND` writer - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
pub type SecPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` reader - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
pub type CtrlEdcVbussPendR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` writer - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
pub type CtrlEdcVbussPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU8` reader - 31:2\\]
Reserved"]
pub type Nu8R = crate::FieldReader<u32>;
#[doc = "Field `NU8` writer - 31:2\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
    #[inline(always)]
    pub fn sec_pend(&self) -> SecPendR {
        SecPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
    #[inline(always)]
    pub fn ctrl_edc_vbuss_pend(&self) -> CtrlEdcVbussPendR {
        CtrlEdcVbussPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
    #[inline(always)]
    #[must_use]
    pub fn sec_pend(&mut self) -> SecPendW<SecStatusReg0Spec> {
        SecPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_pend(&mut self) -> CtrlEdcVbussPendW<SecStatusReg0Spec> {
        CtrlEdcVbussPendW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<SecStatusReg0Spec> {
        Nu8W::new(self, 2)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecStatusReg0Spec;
impl crate::RegisterSpec for SecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for SecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for SecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_STATUS_REG0 to value 0"]
impl crate::Resettable for SecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
