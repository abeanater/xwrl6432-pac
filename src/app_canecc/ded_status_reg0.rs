#[doc = "Register `DED_STATUS_REG0` reader"]
pub type R = crate::R<DedStatusReg0Spec>;
#[doc = "Register `DED_STATUS_REG0` writer"]
pub type W = crate::W<DedStatusReg0Spec>;
#[doc = "Field `DED_PEND` reader - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
pub type DedPendR = crate::BitReader;
#[doc = "Field `DED_PEND` writer - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
pub type DedPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` reader - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
pub type CtrlEdcVbussPendR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` writer - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
pub type CtrlEdcVbussPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU12` reader - 31:2\\]
Reserved"]
pub type Nu12R = crate::FieldReader<u32>;
#[doc = "Field `NU12` writer - 31:2\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
    #[inline(always)]
    pub fn ded_pend(&self) -> DedPendR {
        DedPendR::new((self.bits & 1) != 0)
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
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend."]
    #[inline(always)]
    #[must_use]
    pub fn ded_pend(&mut self) -> DedPendW<DedStatusReg0Spec> {
        DedPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_pend(&mut self) -> CtrlEdcVbussPendW<DedStatusReg0Spec> {
        CtrlEdcVbussPendW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<DedStatusReg0Spec> {
        Nu12W::new(self, 2)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedStatusReg0Spec;
impl crate::RegisterSpec for DedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for DedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for DedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_STATUS_REG0 to value 0"]
impl crate::Resettable for DedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
