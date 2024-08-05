#[doc = "Register `TCCFG` reader"]
pub type R = crate::R<TccfgSpec>;
#[doc = "Register `TCCFG` writer"]
pub type W = crate::W<TccfgSpec>;
#[doc = "Field `FIFO_SIZE_PARAMETERIZATION` reader - 2:0\\]
Fifo Size Parameterization"]
pub type FifoSizeParameterizationR = crate::FieldReader;
#[doc = "Field `FIFO_SIZE_PARAMETERIZATION` writer - 2:0\\]
Fifo Size Parameterization"]
pub type FifoSizeParameterizationW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUS_WIDTH_PARAMETERIZATION` reader - 5:4\\]
Bus Width Parameterization"]
pub type BusWidthParameterizationR = crate::FieldReader;
#[doc = "Field `BUS_WIDTH_PARAMETERIZATION` writer - 5:4\\]
Bus Width Parameterization"]
pub type BusWidthParameterizationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_REGISTER_FIFO` reader - 9:8\\]
Dst Register FIFO Depth Parameterization"]
pub type DstRegisterFifoR = crate::FieldReader;
#[doc = "Field `DST_REGISTER_FIFO` writer - 9:8\\]
Dst Register FIFO Depth Parameterization"]
pub type DstRegisterFifoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Fifo Size Parameterization"]
    #[inline(always)]
    pub fn fifo_size_parameterization(&self) -> FifoSizeParameterizationR {
        FifoSizeParameterizationR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Bus Width Parameterization"]
    #[inline(always)]
    pub fn bus_width_parameterization(&self) -> BusWidthParameterizationR {
        BusWidthParameterizationR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Dst Register FIFO Depth Parameterization"]
    #[inline(always)]
    pub fn dst_register_fifo(&self) -> DstRegisterFifoR {
        DstRegisterFifoR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Fifo Size Parameterization"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_size_parameterization(&mut self) -> FifoSizeParameterizationW<TccfgSpec> {
        FifoSizeParameterizationW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Bus Width Parameterization"]
    #[inline(always)]
    #[must_use]
    pub fn bus_width_parameterization(&mut self) -> BusWidthParameterizationW<TccfgSpec> {
        BusWidthParameterizationW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Dst Register FIFO Depth Parameterization"]
    #[inline(always)]
    #[must_use]
    pub fn dst_register_fifo(&mut self) -> DstRegisterFifoW<TccfgSpec> {
        DstRegisterFifoW::new(self, 8)
    }
}
#[doc = "TC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TccfgSpec;
impl crate::RegisterSpec for TccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccfg::R`](R) reader structure"]
impl crate::Readable for TccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tccfg::W`](W) writer structure"]
impl crate::Writable for TccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCFG to value 0"]
impl crate::Resettable for TccfgSpec {
    const RESET_VALUE: u32 = 0;
}
