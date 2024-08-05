#[doc = "Register `HWACCREG3` reader"]
pub type R = crate::R<Hwaccreg3Spec>;
#[doc = "Register `HWACCREG3` writer"]
pub type W = crate::W<Hwaccreg3Spec>;
#[doc = "Field `CM42ACCTRIG` reader - 0:0\\]
Software trigger bit: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 001b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
pub type Cm42acctrigR = crate::BitReader;
#[doc = "Field `CM42ACCTRIG` writer - 0:0\\]
Software trigger bit: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 001b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
pub type Cm42acctrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `CM42DMATRIG` reader - 31:16\\]
Override accelerator Trigger to DMA.Can be used for triggering the first and second DMA transfer thorugh processor"]
pub type Cm42dmatrigR = crate::FieldReader<u16>;
#[doc = "Field `CM42DMATRIG` writer - 31:16\\]
Override accelerator Trigger to DMA.Can be used for triggering the first and second DMA transfer thorugh processor"]
pub type Cm42dmatrigW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software trigger bit: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 001b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
    #[inline(always)]
    pub fn cm42acctrig(&self) -> Cm42acctrigR {
        Cm42acctrigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override accelerator Trigger to DMA.Can be used for triggering the first and second DMA transfer thorugh processor"]
    #[inline(always)]
    pub fn cm42dmatrig(&self) -> Cm42dmatrigR {
        Cm42dmatrigR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software trigger bit: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 001b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
    #[inline(always)]
    #[must_use]
    pub fn cm42acctrig(&mut self) -> Cm42acctrigW<Hwaccreg3Spec> {
        Cm42acctrigW::new(self, 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Hwaccreg3Spec> {
        NuW::new(self, 1)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override accelerator Trigger to DMA.Can be used for triggering the first and second DMA transfer thorugh processor"]
    #[inline(always)]
    #[must_use]
    pub fn cm42dmatrig(&mut self) -> Cm42dmatrigW<Hwaccreg3Spec> {
        Cm42dmatrigW::new(self, 16)
    }
}
#[doc = "HWACCREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg3Spec;
impl crate::RegisterSpec for Hwaccreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg3::R`](R) reader structure"]
impl crate::Readable for Hwaccreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg3::W`](W) writer structure"]
impl crate::Writable for Hwaccreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG3 to value 0"]
impl crate::Resettable for Hwaccreg3Spec {
    const RESET_VALUE: u32 = 0;
}
