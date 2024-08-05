#[doc = "Register `HWACCREG2` reader"]
pub type R = crate::R<Hwaccreg2Spec>;
#[doc = "Register `HWACCREG2` writer"]
pub type W = crate::W<Hwaccreg2Spec>;
#[doc = "Field `DMA2ACCTRIG` reader - 15:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 16-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
pub type Dma2acctrigR = crate::FieldReader<u16>;
#[doc = "Field `DMA2ACCTRIG` writer - 15:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 16-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
pub type Dma2acctrigW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 16-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
    #[inline(always)]
    pub fn dma2acctrig(&self) -> Dma2acctrigR {
        Dma2acctrigR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 16-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set."]
    #[inline(always)]
    #[must_use]
    pub fn dma2acctrig(&mut self) -> Dma2acctrigW<Hwaccreg2Spec> {
        Dma2acctrigW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Hwaccreg2Spec> {
        NuW::new(self, 16)
    }
}
#[doc = "HWACCREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg2Spec;
impl crate::RegisterSpec for Hwaccreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg2::R`](R) reader structure"]
impl crate::Readable for Hwaccreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg2::W`](W) writer structure"]
impl crate::Writable for Hwaccreg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG2 to value 0"]
impl crate::Resettable for Hwaccreg2Spec {
    const RESET_VALUE: u32 = 0;
}
